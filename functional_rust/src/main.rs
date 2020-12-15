fn main() {
    
    // teste de programacao funcional com rust

    // funcao generica comum 
    fn sum(val1:i32,val2:i32) -> i32 {
        val1+val2
    }


    // passando uma funcao como parametro 
    fn operation<F>(val1:i32,val2:i32,operation_function:F) -> i32 where F: Fn(i32,i32) -> i32{
            // o where serve para definir o tipo so parametro generico F
            operation_function(val1,val2)
        }
    
    // testando

    println!("Rodando a funcao operacao com os parametros 1 e 3 e passando a funcao soma como parametro - {}",operation(1,3,sum));


    // funcoes anonimas (closures)
    // sintaxe
    let _sqrt=|x:i32| x*x; // o valor entre || delimita os parametros e o resto delimita o corpo da funcao


    // pode ser necessario usar a sintaxe mais completa para a closure (as vezes o compilador nao consegue inferir tudo)
    let _sqrt_simples = |x:i32|->i32 {x*x}; // as chaves sao necessarias quando a funcao tem mais de uma linnha


    // uma outra caracteristica interessante das closures e que elas podem acessar dados do ambiente onde ela foi criada 
    // diretamente. Ex:

    const THE_NUMBER:i32=42i32;

    let sum_with_the_number = |x:i32| x+THE_NUMBER ;

    // testando 
    println!("Rodando a funcao sum_with_THE_NUMBER com o parametro 10 - {}",sum_with_the_number(10i32));

    // CLAUSULA MOVE 
    // para resolver problemas com borrowing de das variaveis no ambiente das closures existe a contrucao move que faz o "move"
    // de todo o contexto para a o escopo da closure
    #[derive(Debug)]
    struct User {uid:String,name:String};

    let mut user = User {uid:"10".to_string(),name:"Joaozinho".to_string()};

    // criando a closure sem o move
    let mut muda_nome = |new_name:String| user.name=new_name;
    
    // a struct User continua referenciada pela variavel user
    muda_nome("Enzo Telles Poeta".to_string());
    println!("struct ainda esta associada a variavel user {:?}",user);

    // agora com o move
    let mut muda_nome = move |new_name:String| user.name=new_name;
    muda_nome("Enzo Telles Poeta".to_string());
    // aqui como movemos o escopo a variavel user ja nao tem mais a propriedade da struct User entao o compilator vai reclamar
    //println!("struct ainda esta aqui {:?}",user); // compilador reclama do borrowing via move






}
