fn main() {

    let text =String::from("Hello, world!");

    println!("{}",a_function(text));

    let tup:(i32,f64,i16) =(1,3.2,4);

    let (x,y,z) = tup;

    println!("recuperando valores de uma tupla -- x: {} , y: {} , z:{}",x,y,z);


    let a = [1,2,3,4,5];

    println!("recuperando valores de um array {}",a[1]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("recuperando valores de um array {}",a[0]);

    let y = {
        let x = 3;
        x + 1
    };

    println!("recuperando valores de um variavel que eh definida atraves de uma expressao - {}",y);


}

fn a_function( a_parameter:String )->String
{
    let mut parameter:String = a_parameter;
    
    parameter = parameter + &String::from(" - Funcao do enzo !");

    parameter
    
    
}

