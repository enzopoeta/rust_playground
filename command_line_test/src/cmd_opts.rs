extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CommandLineOptions{
    #[structopt(short="m", long="message")]
    /// Mensagem de teste
    pub message:String,

}