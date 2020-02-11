fn main() {
    println!("Hello, world!");
    /*
        O rust possui um otimo gerenciamento de memoria, mesmo sem ter um garbage colector 
        um dos pilares deste gerenciamento eh o mecanismo the ownership onde qq variavel 
        que esteja na heap (onde o tamanho nao e possivel de saber em tempo de compilacao)
        só pode ter um a unica referencia para exemplificar isso segue o exemplo abaixo :
    */
    let a:String = String::from("Uma String");

    let b:String  = a;

    //println!("{}",a);

    /*
        Se comentarmos o println acima teremos o seguinte erro :


        error[E0382]: borrow of moved value: `a`
        --> src/main.rs:13:19
           |
        9  |     let a:String = String::from("Uma String");
           |         - move occurs because `a` has type `std::string::String`, which does not implement the `Copy` trait
        10 | 
        11 |     let b:String  = a;
           |                     - value moved here
        12 | 
        13 |     println!("{}",a);
           |                   ^ value borrowed here after move

        
        

    */


}
