extern crate structopt;
extern crate stream_viewer;
extern crate crossbeam;
use structopt::StructOpt;
use stream_viewer::cmd_opts::CommandLineOptions;
use crossbeam::channel::bounded;
use crossbeam::channel::unbounded;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use std::thread;

use stream_viewer::io;
use stream_viewer::stats;



fn main() -> Result<(),std::io::Error> {
        
    let cmd_options = CommandLineOptions::from_args();
    
    let infile = cmd_options.input_file.clone();
    let outfile = cmd_options.output_file.clone();

    
    let (stats_sender,stats_receiver):(Sender<usize>, Receiver<usize>)=unbounded();
    let (writer_sender, writer_receiver):(Sender<Vec<u8>>, Receiver<Vec<u8>>)=bounded(1024);

    let read_handle = thread::spawn(move || io::read_loop(&infile, stats_sender, writer_sender));
    
    let stats_handle = thread::spawn(move || stats::compute_bytes_read_loop(stats_receiver));

    let writer_handle = thread::spawn(move || io::write_loop(&outfile, writer_receiver));

    

    // dando um join em cada uma das treads para recuperar o status
    let read_result = read_handle.join().unwrap();
    let stats_result = stats_handle.join().unwrap();
    let writer_result = writer_handle.join().unwrap();
    

    // estoura o erro cado alguma delas nao tenha concluiro corretamente
    read_result?;
    stats_result?;
    writer_result?;

    //se tudo deu certo retorna um result Ok
    Ok(())
}
