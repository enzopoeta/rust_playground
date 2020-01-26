use std::io;
/*
para importar o rand e preciso 
adicionar a referencia a crate dele no Cargo.toml

*/
use rand::Rng;
// importando a enumeracao de Ordenacao
use std::cmp::Ordering;




fn main() {
    println!("Guess the number!");

    // o intervalo da funcao que gera o numero aleatorio e fechado no inicio a fechado no fim
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // no println o {} e a interpolacao de variaveis 
    //println!("The secret number is: {}", secret_number);

    /* 
    instrucao  de loop e preciso ver quais sao a outras formas de fazer isso 
    do jeito que esta ela eh como se fosse um while infinito com um break
    */
    loop {
        println!("Please input your guess.");

        
        /*
        as variaveis no rust por default sao imutaveis se quisermos 
        deixa-las mutaveis eh preciso adicionar o "mut" na frente
        */
        let mut guess = String::new(); //no caso da struct String o new() e  uma funcao associada que +- o conceito de um metodo estatico (escopo de classe)

        
        /* 
        Lendo a entrada do teclado do usuario e guardando o conteudo na variavel guess
        as variaveis ate onde pude entender neste primeiro momento sao sempre passadas por referencia (&)
        e os metodos so retornam as enumerations que podem ser utilizadas para tratar eventuais erros nas
        chamadas de funcao utilizando a clausula expect no caso do read_line a enumeration de retorno e do
        tipo io::Result e poderm retornar um Ok ou um Err
        */
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        
        /*
        Para fazermos a comparacao que vira a seguir e necessario converter a variavel guess 
        que inicialment eh uma string para int vamos aproveitar a mesma variavel guess para fazer isso 
        utilizando o conceito do rust de dar um "shadow" no valor antetior da variavel neste caso se o valor nao
        for um numero uma enumerarion com o erro esta sendo captraca pelo metodo expect
        */
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");


        /*
            O match e uma contrucao que permite comparar valores 
            as enumerarions dele permitem que vc saiba se o valor comparado
            e menor igual ou maior ou menos que o parametro passado
            dentro do match e possivel estabelecer o que fazer em cada uma das situacoes
            no caso do usuario acertar o numero estamos dando um break no loop
        */
        match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => { 
                    println!("You win!");break;
                },
            }
    }



}