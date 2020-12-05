use crate::server::HttpRequestHandler;
use crate::http_protocol::{HttpRequest,HttpResponse,HttpResponseStatus,HttpMethod};
use std::path::{PathBuf,Path};
use std::fs;


pub struct DefaultHttpRequestHandler{

    path:String

}

impl DefaultHttpRequestHandler{

    

    pub fn new(path:String)->Self
    {
        Self{path}
    }

    pub fn read_file(&self,file: &str) -> HttpResponse
    {
       
        println!("PATH ==> {}",self.path);
        println!("PATH ==> {}",file);
        let mut complete_path = PathBuf::new();
        complete_path.push(&self.path);
        
        let mut file_path = Path::new(&file);
        if(file_path.has_root())
        {
            file_path = file_path.strip_prefix("/").unwrap();
        }

        complete_path.push(file_path);

        
        
        println!("Complete path ==> {}",complete_path.display().to_string());

        // verificando se o cara esta tentando explorar problemas de path traversal

        match complete_path.canonicalize()
        {
            Ok(canonical_path)=>{
                if ! canonical_path.starts_with(&self.path)
                {
                    return HttpResponse::new(HttpResponseStatus::OK,Some("<html><body><h1>TRYING PATH TRAVERSAL BITCH !?!?</h1></body></html>".to_string()));
                }
            }
            
            Err(_)=>{return HttpResponse::new(HttpResponseStatus::BAD_REQUEST,None);}
        }


        //fs::read_to_string(complete_path).ok() // o .ok() converte a saida de uma funcao que retorna Result<T> para Option<T>
        if(complete_path.is_file())
        {
            

            match fs::read_to_string(&complete_path){
                // se o arquivo for um binario o read_to_string vai falhar , ai estou tentando usar
                // a request binaria
                Ok(data)=> {return HttpResponse::new(HttpResponseStatus::OK,Some(data))}
                

                Err(_)=>{
                    
                    let file_array = fs::read(&complete_path);
                    if let Ok(data) = file_array{        
                        return HttpResponse::new_bin_response(data)
                    }
                    else
                    {
                        return HttpResponse::new(HttpResponseStatus::BAD_REQUEST, None) 
                    }
                   
                   
                    
                 //return HttpResponse::new(HttpResponseStatus::BAD_REQUEST,None)
                
                }

            }
        }
        else{ HttpResponse::new(HttpResponseStatus::NOT_FOUND,None)}
    }
}



impl HttpRequestHandler for DefaultHttpRequestHandler{
    
    fn handle_http_request(&self , request:& HttpRequest) -> HttpResponse{
        match request.method(){
            HttpMethod::GET=>{
                match request.path(){ // regras de roteamento
                    "/"=>{self.read_file("index.html")}
                    
                    _=>{self.read_file(&request.path())}
                }
                
                
            }
            
            _=>{ HttpResponse::new(HttpResponseStatus::NOT_FOUND,None)}
        }
    }
}

