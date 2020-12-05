extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CommandLineOptions{
    #[structopt(short="hp", long="htdocs_path", default_value = "htdocs")]
    /// Http document root path
    pub htdocs_path:String,

    #[structopt(short="p", long="port", default_value = "8080")]
    pub port:String,

    #[structopt(short="a", long="address", default_value = "127.0.0.1")]
    pub address:String,
}