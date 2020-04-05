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
        // podemos utilizar o match para fazer controle de fuxo usando enums
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => { // quando as enums tem parametros o match ja faz o unwrap automaticamente 
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    // o match pode ser utilizado para qualquer enum inclusive a Option
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1), //  estamos retornando um outro Some somando um ao resultado do parametro
        }
    }

    /*
         Um detalhe dos matches e que eles sao exaustivos o compilador requer que todas as opções validas
         sejam testadas.... por exemplo o match :

         match x {
            Some(i) => Some(i + 1), //  estamos retornando um outro Some somando um ao resultado do parametro
         }

         vai gerar um erro , pois nao estamos tratando a opção None da enum option
    */

    // o match tem um mecanismo para tratar opções que não seão tratadas de maneira explícita com o placeholder "_" como no exemplo abaixo:

    let some_u8_value = 0u8;
    match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (), // para quaisquer outras opções que não as tratadas acima este é o tratamento padrao
    }


    /*
     Por conta de suas caracteristicas (exaustivo) o match pode nao ser a melhor maneira pra 
     um controle de fluxo mais simples para estes casos existe a construcao if let EX:
    */
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // o let if tambem aceita elses...





    


}
