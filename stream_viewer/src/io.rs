use crate::CHUNK_SIZE;

use crossbeam::channel::{Sender,Receiver};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write,ErrorKind};

/**
 esta versao do read eh desenvolvida para rodar como uma thread autonoma que se comunica de
 via channels com as thread de estatistica e de escrita
  */
pub fn read_loop(
    filename_or_empty: &String,
    send_to_stats_thread: Sender<usize>,
    send_to_write_thread: Sender<Vec<u8>>,
) -> Result<(), io::Error> {
    let mut reader: Box<dyn Read> = if !filename_or_empty.is_empty() {
        // quando usamos um objeto trait como neste caso e preciso colocar o dyn na frente (como podemos usar qq coisa que implemente Read o compilador precisa guardar isto na heap , por isso empacotamos tudo em um smartpointer Box)

        Box::new(BufReader::new(File::open(&filename_or_empty)?))
    } else {
        Box::new(io::stdin())
    };

    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let number_of_bytes_read = match reader.read(&mut buffer) {
            Ok(0) => {eprintln!("veio 1 0 saindo");break},
            Ok(x) => x,
            Err(e) => {
                eprintln!("problemas ao ler a entrada escolhida {}", e);
                break;
            }
        };
        // enviando os dados de numero de bytes lidos para o canal da thread de estatisticas
        match send_to_stats_thread.send(number_of_bytes_read) {
            Ok(_) => (),
            Err(e) => {
                eprintln!(
                    "problemas ao enviar os dados para a thread de estatisticas {}",
                    e
                )
            }
        }

        
        // enviado dados para a thread de escrita
        //dbg!(Vec::from(& buffer[..number_of_bytes_read]));
        if send_to_write_thread.is_full()
        {
            eprintln!("canal cheio");
        }
        match send_to_write_thread.send(Vec::from(&mut buffer[..number_of_bytes_read])) {
            Ok(_) => (),
            Err(e) => {
                eprintln!(
                    "problemas ao enviar os dados para a thread de escrita {}",
                    e
                );
                break;
            } //como o envio dos bytes do arquivo eh importante o loop eh interrompido
        }
    }
    let _ = send_to_stats_thread.send(0); // indica para a thread de estatisticas encerrar o loop
    let _ = send_to_write_thread.send(Vec::new()); // indica para a thread de escrita encerrar o loop
    Ok(())
}

pub fn read(filename_or_empty: &String) -> Result<Vec<u8>, io::Error> {
    let mut reader: Box<dyn Read> = if !filename_or_empty.is_empty() {
        // quando usamos um objeto trait como neste caso e preciso colocar o dyn na frente (como podemos usar qq coisa que implemente Read o compilador precisa guardar isto na heap , por isso empacotamos tudo em um smartpointer Box)

        //se for especificado nos parametros de linha de comando ao inves de stdin e out usaremos readers e writers  de arquivo
        /*
        match File::open(&filename_or_empty){
            Ok(file)=> Box::new(BufReader::new(file)),
            Err(e)=> {eprintln!("nao foi possivel ler o arquivo de entrada -- erro -> {}",e);exit(1)}
        }
        */
        Box::new(BufReader::new(File::open(&filename_or_empty)?))
    } else {
        Box::new(io::stdin())
    };

    let mut buffer = [0; CHUNK_SIZE];
    let number_of_bytes_read = reader.read(&mut buffer)?;
    Ok(Vec::from(&buffer[..number_of_bytes_read]))
}


pub fn write_loop(filename_or_empty: &String,receive_from_read_thread:Receiver<Vec<u8>>) -> Result<(),io::Error> {

    eprintln!("entrando thread writer");

    let mut writer: Box<dyn Write> = if !filename_or_empty.is_empty() {
        Box::new(BufWriter::new(File::create(filename_or_empty)?))
    } else {
        Box::new(io::stdout())
    };

    loop{
        //eprintln!("iterando loop write");
        
        let buffer = receive_from_read_thread.recv().unwrap(); // array de bytes vindo da thread de leitura

        if buffer.is_empty() // se o buffer vier vazio eh o sinal para o encerramento do loop
        {
            break;
            eprintln!("buffer feio vazio");
        }
        //eprintln!("recebendo dados do buffer no writer");
        match  writer.write_all(&buffer){

            Ok(_) => {},//return Ok(()),
            Err(e) => {
                if e.kind() != ErrorKind::BrokenPipe{
                eprintln!(
                    "problermas ao escrever os dados para a thread de escrita {}",
                    e
                );return Err(e);//break
                
            }
            
        }

    }}

    Ok(())

}

pub fn write(filename_or_empty: &String, buffer: &[u8]) -> Result<(), io::Error> {
    let mut writer: Box<dyn Write> = if !filename_or_empty.is_empty() {
        Box::new(BufWriter::new(File::create(&filename_or_empty)?))
    } else {
        Box::new(io::stdout())
    };

    writer.write_all(&buffer)?;

    Ok(())
}
