/*
Enumeracoes, tambem conhechidas como enums nos permite definir um novo tipo enumerando suas possives 
variações. 
Enumerações combinadas com dados podem adicionar bastante significado ao codigo
*/

// uma enum simples
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}


// uma enum que pode receber dados
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}



// como usar enums em funcoes
fn route(ip_kind: IpAddrKind) { }

//tambem e possivel utilizar enums detro de structs
#[derive(Debug)]
struct SIpAddr {
    kind: IpAddrKind,
    address: String,
}

//tambem e possivel definir metodos para uma enum atraves da contrucao impl
impl IpAddr {
    fn call(&self) {
        // method body would be defined here
        println!("Ola mundo!");
    }
}


fn main() {
    println!("Hello, world!");


    // setando  valores 
    
    let four = IpAddrKind::V4;
    println!("Loopback ipv4-> {:?}",four);


    let an_address = SIpAddr {
        kind: IpAddrKind::V4,
        address: String::from("192.168.0.1"),
    };
    println!("struct DE IP-> {:?}",an_address);


    let home = IpAddr::V4(127, 0, 0, 1);
    println!("Loopback ipv4-> {:?}",home);
    home.call();

    let loopback = IpAddr::V6(String::from("::1"));
    println!("Loopback ipv6-> {:?}",loopback);
    

    /*
        -- A ENUM OPTION --

        Rust diferente de outras linguagens de programacao nao tem o conceito de null
        Como alternativa a linguagem possui a Enum especial Option<T> definida na standard
        library com a seguinte estrutura :

        enum Option<T> {
            Some(T),
            None,
        }

        





    */








}
