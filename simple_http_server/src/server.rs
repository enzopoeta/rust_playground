use std::convert::TryFrom;

use std::io::ErrorKind;
use std::io::Read;
use std::net::TcpListener;

use crate::http_protocol::{HttpRequest,HttpResponse,HttpResponseStatus,RequestParseError};


/*
    criando o contrato da struct que vai lidar com as requests via custom trait
    no caso da funcao handle_http_bad_request ja e possivel definir um 
    comportamento default pq ela eh muito simples

*/

pub trait HttpRequestHandler{

    fn handle_http_request(&self , request:& HttpRequest) -> HttpResponse;

    fn handle_http_bad_request(&self,error: & RequestParseError) -> HttpResponse
    {
        HttpResponse::new(HttpResponseStatus::BAD_REQUEST,None)
    }   

}




#[derive(Debug)]
pub struct SimpleHttpServer {
    ip_adress: String,
    port: String,
}

impl SimpleHttpServer {
    pub fn new(binding_adress: &str, server_port: &str) -> SimpleHttpServer {
        // tambem poderiamos usar a clausula especial Self aqui {
        SimpleHttpServer {
            ip_adress: binding_adress.to_string(),
            port: server_port.to_string(),
        }
    }

    // pub fn run(&self) // trocando a implementacao de run para aceitar como parametro alguem que implemente nossa trait  HttpRequestHandler
    // tambem seria possivel passar o run sem referencia ja que ele vai subir um loop e quando a funcao sair de escopo vamos querer desalocar toda a memoria
    pub fn run(&self,request_handler: impl HttpRequestHandler)
    {
        println!("server conf : {:?}", self);

        let ip_and_port = format!("{}:{}", self.ip_adress, self.port);
        let listener = TcpListener::bind(&ip_and_port).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::AddrInUse {
                panic!(
                    "Cannot open socket on ip {} with port {}. Address already in use !",
                    self.ip_adress, self.port
                );
            } else {
                panic!(
                    "Cannot open socket on ip {} with port {}. Unknown error -- {:?}",
                    self.ip_adress,
                    self.port,
                    error.kind()
                );
            }
        });

        'serverloop: loop {
            match listener.accept() {
                Ok((mut stream, socket)) => {
                    println!("recebida conexao em {:?} ", socket);
                    //criando um array para receber os dados de de read da uma stream
                    // a funcao read vai receber ele como referencia entao em principio
                    // pode se de qualquer tamanho
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            println!("recebibos {} bytes do cliente", bytes_read);
                            let response = match HttpRequest::try_from(&buffer[..]) {
                                Ok(request) => {
                                    //println!("teste");
                                    println!("Objecto Request retornado -- {:?}",request);

                                    // teste de response

                                    //HttpResponse::new(HttpResponseStatus::OK,Some("<h1>Its working !</h1>".to_string()))
                                    request_handler.handle_http_request(&request)
                               
                                    

                                }
                                Err(e) => {
                                    println!("cannot convert buffer to request {:?}", e);
                                    // alem de printar o erro agora e possivel mandar uma response com o status bad request
                                    request_handler.handle_http_bad_request(& e)
                                    //HttpResponse::new(HttpResponseStatus::BAD_REQUEST,None)
                                }
                            };
                            println!("Tentando enviar a response {}",response);

                            if let Err(e) = response.send(& mut stream){
                                println!("Erro eo enviar a response");
                            }

                            //println!("Request Debug :\n{}",String::from_utf8_lossy(&buffer));
                        }
                        Err(e) => {
                            println!("erro ao receber dados do cliente -- {:?}", e.kind());
                        }
                    }
                }
                Err(e) => {
                    println!("erro {:?} ", e.kind());
                    continue 'serverloop;
                }
            }
        }

        // essa construcao nao retorna o socketaddress
        /*
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("new client!");
                }
                Err(e) => {  }
            }
        }*/
    }
}
