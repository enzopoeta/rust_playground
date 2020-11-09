

pub use method::HttpMethod; // essa declaracaode use permite que o HttpMethod e  HttpRequest possam ser chamados diretamente do http_protocol ( alem das chamadas normais usando o caminho completo)
pub use request::HttpRequest;
pub use request::RequestParseError;
pub mod method;

pub mod request;

