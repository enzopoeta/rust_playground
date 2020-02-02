fn main() {
    
    // exemplo if ( nada de muito diferente de outras liguagens)
    
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // usando if na atribuicao de valor para variaveis
    // (if e uma expressao)
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    // O statement loop repete o bloco de codigo indefinidamente ate que seja 
    // interrompido por um break (que pode ser utilizado para retornar um valor como no exemplo abaixo)

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of result is: {}", result);


    // o rust tambem possui o while, que tambem nao tem grandes diferencas de outras liguagens
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }


    // O for no rust tem como principal funcao iterar colecoes (for each em outras langs)
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }


    // tambem pode se utilizado com o tipo range disponivel na standard lib conforme o exemplo abaixo
    for number in (1..4).rev() { // o rev() faz com que o range seja considerado de traz para frente trazendo um efeito de countdown
        println!("{}!", number);
    }
    




}
