extern crate structopt;

use structopt::StructOpt;




#[derive(StructOpt)]
pub struct CommandLineOptions{
    #[structopt(short="i", long="input_file",default_value="")]
    /// Arquivo de entrada
    pub input_file:String,

    #[structopt(short="o", long="output_file",default_value="")]
    /// Arquivo de saida
     pub output_file:String,


}

