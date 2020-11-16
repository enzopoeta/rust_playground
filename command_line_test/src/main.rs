extern crate command_line_test;
extern crate structopt;
extern crate colored;
use colored::*;
use structopt::StructOpt;


use command_line_test::cmd_opts::CommandLineOptions;

fn main() {

    let cmd_options = CommandLineOptions::from_args();

    let message = cmd_options.message;

    println!("valor do parametro message -- > {}",message.bright_yellow().underline() );
}
