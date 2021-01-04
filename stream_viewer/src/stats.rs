use crossbeam::channel::Receiver;
use std::io::{self};



/**
 esta versao do compute_bytes_read eh desenvolvida para rodar como uma thread autonoma que recebe dados
 via channel da thread de leitura

  */
pub fn compute_bytes_read_loop(receive_from_read_thread:Receiver<usize>) -> Result<(),io::Error>{

    let mut total_bytes_read=0;
    loop{
        let number_of_bytes_read = receive_from_read_thread.recv().unwrap();
        total_bytes_read += number_of_bytes_read;
        eprint!("\rbytes read -- {}", total_bytes_read);
        
        if number_of_bytes_read==0{
            break;
        }


    }

    Ok(())
    
}




pub fn compute_bytes_read(
    number_of_bytes_read: usize,
    total_bytes_read: &mut usize,
    end_of_file:  bool,
) {
    *total_bytes_read += number_of_bytes_read;
    eprint!("\rbytes read -- {}", total_bytes_read); // usando o eprint com o carriage return "\r" a saida e sempre reescrita na tela no mesmo lugar
    if end_of_file {
        println!("");
    }
}
