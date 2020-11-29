use super::method::HttpMethod;
use super::method::InvalidMethodError;
use super::query_string::{QueryString};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;
use std::str::FromStr;


#[derive(Debug)]
pub struct HttpRequest<'buffer> {
    path: & 'buffer str,
    query_string: Option<QueryString<'buffer >>,
    method: HttpMethod,
}

impl<'buffer> HttpRequest<'buffer>{
    pub fn path(&self)-> &str{
        &self.path
    }

    pub fn method(&self)-> &HttpMethod{
        &self.method
    }

    pub fn query_string(&self)-> Option<&QueryString>{
        self.query_string.as_ref()
    }


}


fn get_request_main_parameters(text:&str)->Vec<&str>{
    let lines:Vec<&str> = text.split("\r\n").collect();
    if lines.len()>0
    {
        lines[0].split(" ").collect()
    }
    else
    {
        lines
    }

}

/* implementacao original
fn get_query_string(request_complete_path:&str)->Option<String> {
        
    let mut result:Option<String> = None;
    
    match request_complete_path.find("?")
    {
        Some(index)=>{
            result = Some((request_complete_path[index+1..]).to_string()) // usamos index+1 para definir a slice para nao incluir o ? na string isso so funciona (usar o index) pq o ? tem 1 byte

        },
        None=>{}
    }
    
    result
}
*/

fn get_query_string(request_complete_path:&str)->Option<QueryString> {

    let mut result:Option<QueryString> = None;
    let parameters:Vec<&str> = request_complete_path.split("?").collect();
    if parameters.len() >1{
        result=Some(QueryString::from(parameters[1]))
    }

    result

}

fn get_path(request_complete_path:&str)->&str {
        
    let result:Vec<&str> = request_complete_path.split("?").collect();
    
    
    result[0]
}


// fica a dica eh sempre interessante verificar as traits da stdlib antes de sair implementando as coisas
// neste caso era necessario um metodo para converter o buffer [u8] e e verificamos que ja existe uma trait
// que faz esse tipo de conversao no caso temos a std::convert::TryFrom que diferente da From pode falhar e
// retorna um result

/*
A sintaxe 'buffer nos retonos de tipo especifica um lifetime para os parametros de entrada e saida
isso significa que as regioes de memoria do buffer passado como parametro de entrada que estiverem 
sendo usadas terao seu tempo de vida atrelado ao objeto request de retorno. Isso permite que usemos
o buffer sem replicar a informacao e nao permite que tenhamos dangling references, mesmo sem ter um mecanismo 
de garbage collector
*/
impl<'buffer> TryFrom<& 'buffer [u8]> for HttpRequest<'buffer> {
    type Error = RequestParseError; // para esta trait e preciso definir qual e o tipo de erro

    

    
    fn try_from(buffer: &'buffer [u8]) -> Result<HttpRequest<'buffer>, Self::Error> {
        
        let request = str::from_utf8(buffer)?;

        // Esta implementacao funciona mas e possivel fazer um implementacao mais sucinta convertendo os erros
        // que estouram para a enum de erros da funcao para isso temos que implementar a Trait From no RequestParseError
        /*
        match str::from_utf8(buffer)
         {
             Ok(string)=>{println!("teste")},
             Err(e)=>RequestParseError::InvalidEncoding
         }
         */

        //println!("request recebida {:?}",request);
        let request:Vec<&str> = get_request_main_parameters(&request);//request.split(" ").collect();
        if request.len() != 3{
            return Err(RequestParseError::InvalidRequest);
        }

        let method = request[0];
        let path = request[1];
        let protocol = request[2];

        if protocol != "HTTP/1.1"
        {
            return Err(RequestParseError::InvalidProtocol);
        }
        
        //println!("Vetor de parametros {:?}",request);
        
        //HttpRequest {}

        let method = HttpMethod::from_str(method)?;
        // tambem e possivel utilizar a trait parse que no fim das contas vai utilizar a implementacao
        // da trait FromStr no HttpMethod
        // let method:HttpMethod = method.parse()?;
       
        let q_string = get_query_string(path);

        let path = get_path(path);

        
        
        let req = HttpRequest {path:path ,query_string:q_string,method:method};
        //println!("request debug -- {:?}",req);
        
        Ok(req)        
    
    

        //unimplemented!(); // essa maCRO faz com que o compilador pare de reclamar quando nao queremos implementar algo imediatamente

    }
}


pub enum RequestParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl RequestParseError {
    fn message(&self) -> &str {
        match self {
            RequestParseError::InvalidRequest => "Invalid Request",
            RequestParseError::InvalidEncoding => "Invalid Encoding",
            RequestParseError::InvalidProtocol => "Invalid Protocol",
            RequestParseError::InvalidMethod => "Invalid Method",
        }
    }
}

// para implementar a trait de erro na enum que criamos para isso e preciso tamplem implementar a trait debug e a trait display

impl Display for RequestParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for RequestParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for RequestParseError {
    fn from(_: Utf8Error) -> Self {
        // o _ serve para ignorar o parametro e o compilador nao encher o saco
        RequestParseError::InvalidEncoding
    }
}


impl From<InvalidMethodError> for RequestParseError {
    fn from(_: InvalidMethodError) -> Self {
        // o _ serve para ignorar o parametro e o compilador nao encher o saco
        RequestParseError::InvalidMethod
    }
}

impl Error for RequestParseError {
    //unimplemented!();
}
