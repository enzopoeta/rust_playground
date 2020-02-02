fn main() {

    let text =String::from("Hello, world!");

    // chamando a funcao a_function
    println!("{}",a_function(text));

    // declaracao de uma tupla
    let tup:(i32,f64,i16) =(1,3.2,4);

    // como extrair os valores de uma tupla
    let (x,y,z) = tup;

    println!("recuperando valores de uma tupla -- x: {} , y: {} , z:{}",x,y,z);

    // No rust arrays sao fixos (nao podem crescer ou diminuir o tamanho)
    
    //uma das maneiras de declarar um array
    let a = [1,2,3,4,5];
    println!("recuperando valores de um array {}",a[1]);
    // outra sintaxe de declarao de array (dando um shadowing na primeira versao)
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("recuperando valores de um array {}",a[0]);

    
    // tambem é possivel e fazer a aribuicao de uma variavel com o resultado de uma expressao
    let y = {
        let x = 3;
        x + 1
    };

    println!("recuperando valores de um variavel que eh definida atraves de uma expressao - {}",y);


}

// declaracao de uma funcao com retorno no rust os parametros da funcao precisam ter os tipos 
// declarados o tipo do retorno e declsarado pelo statement "->""
fn a_function( a_parameter:String )->String
{
    let mut parameter:String = a_parameter;
    
    parameter = parameter + &String::from(" - Funcao do enzo !");

    parameter // no rust nao existe um "return" para o retorno da funcao ... basta nao colocar o ";"  no ultimo statement

}

