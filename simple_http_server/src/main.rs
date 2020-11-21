extern crate simple_http_server;

use std::env;
use simple_http_server::server::SimpleHttpServer;
use simple_http_server::http_protocol::HttpRequest;
use simple_http_server::http_protocol::HttpMethod;
use simple_http_server::http_request_handler::DefaultHttpRequestHandler;

fn main() {


    // verificando se o usuario esta setando um storage customizado para as paginas
    let path=
    match(env::current_dir())
    {
        Ok(mut path_buffer)=>{
            path_buffer.push("htdocs");
            path_buffer.as_path().display().to_string()
        },
        Err(_)=>{
            panic!("Cannot obtain the current dir");
        }
    };

    

    println!("{}",path);


    let server = SimpleHttpServer::new("127.0.0.1", "8080");
    //println!("server conf : {:?}", server);
    server.run(DefaultHttpRequestHandler::new(path)); 
}





