use crate::server::HttpRequestHandler;
use crate::http_protocol::{HttpRequest,HttpResponse,HttpResponseStatus,HttpMethod};


pub struct DefaultHttpRequestHandler{

    path:String

}

impl DefaultHttpRequestHandler{

    pub fn new(path:String)->Self
    {
        Self{path}
    }
}



impl HttpRequestHandler for DefaultHttpRequestHandler{
    
    fn handle_http_request(&self , request:& HttpRequest) -> HttpResponse{
        match request.method(){
            HttpMethod::GET=>{
                match request.path(){ // regras de roteamento
                    "/"=>{{HttpResponse::new(HttpResponseStatus::OK,Some("<h1>Helo World</h1>".to_string()))}}
                    
                    _=>{HttpResponse::new(HttpResponseStatus::NOT_FOUND,None)}
                }
                
                
            }
            
            _=>{ HttpResponse::new(HttpResponseStatus::NOT_FOUND,None)}
        }
    }
}

