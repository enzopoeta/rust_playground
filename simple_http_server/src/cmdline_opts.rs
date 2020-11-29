extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CommandLineOptions{
    #[structopt(short="p", long="htdocs_path", default_value = "htdocs")]
    /// Http document root path
    pub htdocs_path:String,
}