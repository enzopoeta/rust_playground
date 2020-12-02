use std::io::Result as IOResult;
use std::fmt::{Display,Formatter,Result as FmtResult};
use std::io::Write;
use std::fs;

pub struct HttpResponse{
    status_code:HttpResponseStatus,
    body: Option<String>,
    bin_body: Option<Vec<u8>>
}

impl HttpResponse{

    pub fn new_bin_response( bin_file_path:&str) -> HttpResponse {
        
        let file_arrray = fs::read(bin_file_path);
        if let Ok(data) = file_arrray{        
            HttpResponse {status_code:HttpResponseStatus::OK,body:None,bin_body:Some(data)
            }
        }
        else
        {
            HttpResponse {status_code:HttpResponseStatus::BAD_REQUEST,body:None,bin_body:None}
        }

        //unimplemented!();
    }

    
    
    
    pub fn new(status_code:HttpResponseStatus, body:Option<String>) -> HttpResponse {
        HttpResponse {status_code,body,bin_body:None}
    }
    //pub fn send(&self,tcp_stream: & mut TcpStream ) -> IOResult<()>{ funcao original chamando um tipo concreto TcpStream
    //pub fn send(&self,tcp_stream: & mut dyn Write ) -> IOResult<()>{ funcao com dinamic dispatch : colocando a trait Write poderiamos passar como parametro qq classe 
    //concreta que implementa essa trait ... no entanto isso causa um overhead para que o compilador 
    //localize em runtime qual e a implementacao necessaria para cada chamada (V-tables)
    pub fn send(&self,tcp_stream: & mut impl Write ) -> IOResult<()>{ // exemplo de um static dispatch... neste caso
        // o compilador vai verificar onde a funcao esta sendo chamada , e em cada caso ele recria a funcao com o tipo concreto que esta sendo usado
        // evitando assim qualquer tipo de overhead ... por outro lado aumenta o tamanho do binario e o tempo de compilacao
        //o uso do dispatch e interessante para deixar a funcao mais flexivel por exemplo para fazermos testes 

        let body = match &self.body{
            Some(data)=>data,
            None=>"",
        };

        // o tcp stream tem a trait Write portanto podemos usar a  a macro write! para escrever na stream
                                    //write!(stream,"{}",response);
        // formatando  a impressao da saida HTTP como deve ser na especificacao
        if self.body != None {
        write!(tcp_stream,"HTTP/1.1 {} {}\r\n\r\n{}",self.status_code,self.status_code.get_reason(),body)
        }
        else{
        match &self.bin_body
         {
              Some(body)=>{
                    
                let file_bytes:&[u8] = body.as_slice();
                let response_text = format!("HTTP/1.1 {} {}\r\n\r\n",self.status_code,self.status_code.get_reason());
                tcp_stream.write_all(response_text.as_bytes());
                tcp_stream.write_all(file_bytes);
                tcp_stream.flush()
                    
                }
                None=>write!(tcp_stream,"HTTP/1.1 {} {}\r\n\r\n",HttpResponseStatus::BAD_REQUEST,HttpResponseStatus::BAD_REQUEST.get_reason())
         }
        }
    }

            //write!(tcp_stream,"HTTP/1.1 {} {}\r\n\r\n{}",self.status_code,self.status_code.get_reason(),file_bytes)

        //unimplemented!();
    //}

}

impl Display for HttpResponse{

    fn fmt(&self,f: &mut Formatter) -> FmtResult{
        let body = match &self.body{
            Some(data)=>data,
            None=>"",
        };

        // formatando  a impressao da saida HTTP como deve ser na especificacao
        write!(f,"HTTP/1.1 {} {}\r\n\r\n{}",self.status_code,self.status_code.get_reason(),body)
    
    }

}



#[derive(Copy,Clone)] // estou usando o atributo derive para "injetar" as implementacoes default das traits Copy e Clone necessarias para a funcao fmt da emun de status
pub enum HttpResponseStatus{
    OK = 200,
    BAD_REQUEST = 400,
    NOT_FOUND = 404,

}

impl HttpResponseStatus{
    fn get_reason(&self) -> &str {

        match self
        {
            Self::OK=>{ "OK"},
            Self::BAD_REQUEST=>{"Bad Request"},
            Self::NOT_FOUND=>{"Not Found"},

        }
    }
}

impl Display for HttpResponseStatus {
    fn fmt(&self,f: &mut Formatter) -> FmtResult{
        write!(f,"{}",*self as u16)
    }
}

