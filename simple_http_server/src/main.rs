extern crate simple_http_server;
use std::fs;
use std::path::Path;
use std::env;
use std::process::*;

use simple_http_server::server::SimpleHttpServer;
use simple_http_server::http_request_handler::DefaultHttpRequestHandler;
use simple_http_server::cmdline_opts::CommandLineOptions;
use structopt::StructOpt;

fn main(){

    let cmd_options = CommandLineOptions::from_args();

    // verificando se o usuario esta setando um storage customizado para as paginas
    let mut path=
    match(env::current_dir())
    {
        Ok(mut path_buffer)=>{
            path_buffer.push("htdocs");
            if ! path_buffer.is_dir(){
                match fs::create_dir(path_buffer.as_path().display().to_string())
                {
                    Ok(_)=>{

                    },
                    Err(_)=>{
                        //eprintln!("Invalid htdocs path !");
                        //ExitCode::FAILURE.report()                    }
                        //panic!("cannot create the htdocs folder");
                        eprintln!("ERROR -- Cannot create default htdocs folder ! Check current folder permission !");   
                        exit(-1);
            
                }
            }

        }
            path_buffer.as_path().display().to_string()
        },
        Err(_)=>{
            //panic!("Cannot obtain the current dir");
            eprintln!("ERROR -- Cannot obtain the current dir !");   
            exit(-1);

        }
    };

    // verificando se o usuario passou um diretorio diferente do default
    if "htdocs".to_string() != cmd_options.htdocs_path{
        if ! Path::new(&cmd_options.htdocs_path).is_dir() {
            eprintln!("ERROR -- Invalid path for htdocs folder !");   
            exit(-1);

        }
        else{
            path=cmd_options.htdocs_path;
        }
    }

    

    println!("{}",path);


    let server = SimpleHttpServer::new(&cmd_options.address,&cmd_options.port );
    //println!("server conf : {:?}", server);
    server.run(DefaultHttpRequestHandler::new(path)); 

    
}





