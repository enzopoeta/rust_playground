

pub use method::HttpMethod; // essa declaracaode use permite que o HttpMethod e  HttpRequest possam ser chamados diretamente do http_protocol ( alem das chamadas normais usando o caminho completo)
pub use request::HttpRequest;
pub use request::RequestParseError;
pub use query_string::{QueryStringValue,QueryString};
pub use response::{HttpResponse,HttpResponseStatus};


pub mod method;
pub mod query_string;
pub mod request;
pub mod response;

