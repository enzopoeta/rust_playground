use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    // exitem 2 maneiras basicas de se gerenciar error em rust...
    // usando panic uo lifando com a enum Result 
    

    // Panic :
    // normalmente utilizamos um panic para erros dos quais o software NAO deveria se
    //recuperar (como por exemplo no caso de acesso a uma posicao inexistente em um vetor)

    let vet = vec![1,2,3];
    vet[0];

    //vet[1023];  

    // descomentar a linha acima gera um erro do tipo:
    /*
    thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 1023', src/main.rs:11:5
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    se quisermos verificar o stacktrace completo do erro eh so setar a variavel RUST_BACKTRACE com o valor 1
    */


    //Result :
    /* 
    Normalmente queremos que nosso software trate e se recupere de erros para isso ao inves de exceptions
    como em outras linguagens temos a enum Result do Rust
    
    Conforme podemos observar a enum Result tem duas variacoes :

    enum Result<T, E> {
    Ok(T),
    Err(E),
    }

    T e E sao tipos genericos e T representa o tipo de retorno em caso de sucesso e E reprenta o Erro caso ele aconteca

    Abaixo temos um exemplo de erro  de tratamento de erro com result em mais detalhes :
       
    */

    
    let f = File::open("hello.txt"); // a lib de abertura de arquivo retorna um result 

    // um dos jeitos de lidar com o resultado da abertura e utilizando a clausula match
    println!("{:?}",f);

    
    let f = match f {
        Ok(file) => file, //se o result tiver um resultado estamos retornando-o ( o match ja faz o unwrap automaticamente)
        // no caso de um erro estamos abrindo um outro match para descobrir o tipo
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") { // caso o erro for do tipo NotFound (ou seja o arquivo nao existe) estamos
                //tentando criar o arquivo
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),// se mesmo assim nao der certo estamos gerando um panic e printando o erro
            },
            other_error=>  {
                panic!("Problem opening the file: {:?}", other_error) // para outros tipos de erro estamos saindo com um panic e printando o erro
            }
        },
    };

    println!("{:?}",f);


    // tambem e possivel fazer a mesma coisa utilizando uma sintaxe de clojures para deixar o codigo mais conciso

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    println!("{:?}",f);


    // Atalhos para situacoes do tipo panic on error
    //a emun Result tem metodos interessantes para este tipo de operacao ... são eles 
    
    // unwrap() , Ex:
    
    //let f = File::open("hello2.txt").unwrap();

    // descomentar a linha acima geraria um erro do tipo
    //thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:91:38

    // tambem existe o expect de que funciona da mesma forma que o unwrap() so que nos permite especificar a mensagem de erro. Ex:

    //let f = File::open("hello2.txt").expect("Failed to open hello.txt");

}


// propagacao de erros 
// muitas vezes nao queremos tratar o erro dentro da funcao mas sim propaga-lo para quem fez a chamada :
// segue um exemplo usando match

//esta funcao abre um e le uma string dentro dele 
// e possivel observar os matchs para cada operacao
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// no entanto  a sintaxe padrao do match e bastante verbosa
// podemos escrever bem menos codigo usando o operador de propagacaode de erros ? conforme o exemplo abaixo :
// OBS: O OPERADOR ? so pode ser utilizado em funcoes que retornam valor
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// ou ainda de maneira mais concisa
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}














