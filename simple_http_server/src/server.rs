use crate::http_protocol::HttpRequest;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::ErrorKind;
use std::io::Read;
use std::net::TcpListener;

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

    pub fn run(&self)
    // tambem seria possivel passar o run sem referencia ja que ele vai subir um loop e quando a funcao sair de escopo vamos querer desalocar toda a memoria
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
                            match HttpRequest::try_from(&buffer[..]) {
                                Ok(request) => {
                                    //println!("teste");
                                    println!("Objecto Request retornado -- {:?}",request);
                                }
                                Err(e) => {
                                    println!("cannot convert buffer to request {:?}", e);
                                }
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
