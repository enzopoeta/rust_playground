extern crate code_organization;

// para nao termmos a necessidade  de chamar o caminho commpleto todas as vezes 
// podemos usar a construcao use
use code_organization::ipv4::inner_module;

// tambem e possivel utilizar aliases para evitar ambiguidades
use code_organization::ipv4::inner_module as bullshit;

// quando utilizamos u use para funcoes normalmente nao chegamos so ao nivel do modulo da mesma
// mas quando usamos o use para structs enums e tuplas deve-se chegar o nivel do objeto em si



fn main(){
    // exemplo de chamada com o caminho completo
    code_organization::ipv4::inner_module::connect();
    
    // exemplo simpleficado com o use
    inner_module::connect();

    // exemplo com alias
    bullshit::connect();

}