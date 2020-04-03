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

        neste a ideia e empacotar o resultado de uma operacao em some quando existe um valor existente a ser retornado na funcao
        e none quando não existe nenhum valor isto na pratica elimina a necessidade de checagem de valores nulos 
        tradicional ja que o none eh seguem alguns exemplos de utilização do opttion
    */

    let absent_number: Option<i32> = None;
    let some_string = Some("a string");
    let some_number = Some(5); // a enum option ja tem o seu contexto importado por default portanto nao temos que utilizar Option::

    // nao podemos por exemplo somar uma numero com a variavel some_number porque ela e to tipo some no entanto podemos 
    // "desenvelopar" o valor usando a funcao unwrap -- lembrar de ver a pagina de docomentecao de option
    let a_sum = 5 + some_number.unwrap(); // a enum option possui uma serie de metodos uteis ... por exemplo podemos fazer o unpack um valor de some invocando a funcao unwrap
    println!("soma de 5 + o valor que tiramos de dentro do some<t> --> {}",a_sum);


    // eh possivel utilizar a construcao match para tratar o valor de enums ..,.para as enums de exemplo da documentacao : 

    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
    }

    
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    // poderiamos ter a funcao que verifica  cada tipo possivo de coin e realiza uma operacao de acordo com cada um deles
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    // podemos utilizar


}
