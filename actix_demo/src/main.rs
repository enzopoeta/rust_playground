use actix_web::middleware::Logger;
use actix_web::middleware::Compress;
use actix_web::{get,web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
struct User {
    user_id: String,
    user_name: String,
    user_role: String,
}

async fn get_sample(req: HttpRequest) -> impl Responder {
    match req.match_info().get("name") {
        Some(name) => Some(web::Json(User {
            user_id: name.to_string(),
            user_name: "Enzo Telles Poeta".to_string(),
            user_role: "admin".to_string(),
        })),
        None => None,
    }
}

async fn get_data_frompath(req: HttpRequest,name: web::Path<String>,
) -> impl Responder {
    format!("Hello {} ", name)
}

async fn post_form_sample(form: web::Form<User>) -> impl Responder {
    format!("Hello {} from {}!", form.user_name, form.user_role)
}

async fn post_json_sample(form: web::Json<User>) -> impl Responder {
    format!("Hello {} from {}!", form.user_name, form.user_role)
}

#[derive(Serialize,Deserialize)]
struct QueryStringParams {
    param1:String,
    param2:String
}

//#[get("/querystring")] // com esta anotacao nao eh preciso colocar as coisas na inicializacao de rotas
async fn query_string_sample(info: web::Query<QueryStringParams>) -> impl Responder {
    format!("Welcome {}!", info.param1)
}

async fn header_test(req:HttpRequest)-> impl Responder{
    let req_headers = req.headers();

    let basic_auth_header = req_headers.get("Authorization");
    match basic_auth_header {
        Some(header)=>{
            let basic_auth: &str = header.to_str().unwrap();
            format!("Authorization header value : {}",basic_auth)
        }
        None=>{
            format!("Authorization header not found")
        }
    }
    
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("debug")).init();

    // carregando certificado ssl
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    ssl_builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    ssl_builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) //middleware de log
            .wrap(Logger::new("%a %{User-Agent}i")) ////middleware de log
            .wrap(Compress::default()) // middleware de compressao
            .route("/", web::get().to(get_sample))
            .route("/requestmatchinfo/{name}", web::get().to(get_sample))
            .route("/querystring", web::get().to(query_string_sample))
            .route("/path/{name}", web::get().to(get_data_frompath))
            .route("/post_form", web::post().to(post_form_sample))
            .route("/post_json", web::post().to(post_json_sample))
            .route("/header_test", web::get().to(header_test))

            //.route("/post_form", web::post().to(post_form_sample))
    })
    //.bind("127.0.0.1:8080")?
    .bind_openssl("127.0.0.1:8080", ssl_builder)?
    .run()
    .await
}
