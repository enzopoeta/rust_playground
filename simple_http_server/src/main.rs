extern crate simple_http_server;


use simple_http_server::server::SimpleHttpServer;
use simple_http_server::http_protocol::HttpRequest;
use simple_http_server::http_protocol::HttpMethod;

fn main() {
    let server = SimpleHttpServer::new("127.0.0.1", "8080");
    //println!("server conf : {:?}", server);
    server.run();
}





