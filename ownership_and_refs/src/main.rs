fn main() {
    println!("Hello, world!");
    /*
        O rust possui um otimo gerenciamento de memoria, mesmo sem ter um garbage collector 
        um dos pilares deste gerenciamento eh o mecanismo the ownership onde qq variavel 
        que esteja na heap (onde o tamanho nao e possivel de saber em tempo de compilacao)
        só pode ter um unico "dono" para exemplificar isso segue o exemplo abaixo :
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

      
    Isso acontece porque o rust nao trabalha por padrao com a copia de variaveis da heap e por uma questao de performance e seguranca ele 
    so permite que o dado seja referenciado por uma unica variavel fazendo assim com que a referencia relacionada com a variavel anterior 
    seja anulada em detrimento da atribuicao atual, mesmo que a variavel esteja com o escopo valido. Para forcar uma copia de dados da heap 
    podemos usar  a funcao clone conforme o exemplo abaixo
    */

   let s1 = String::from("hello");
   let s2 = s1.clone();

   println!("s1 = {}, s2 = {}", s1, s2);


   /*
   Em variaveis que sao definidas inteiramente na stack (tamanho connhecido em tempo de compilacao) a copia de valores 
   e feita automaticamente na atribuicao. Ex:
   */

   let a = 1;
   let b:i32;
   b=a;

   println!("a = {}, b = {}", a, b);
   //como podemos observar ambas as variaveis conservaram seu valor ( na verdade o valor de a foi copiado para b)

   /*
         ownership e funcoes --
         O mecanismo e parecido com a atribuicao de variaveis... Ao passar uma variavel que nao fica na stack
         voce esta cedendo a propriedade para a funcao ... se nao 

   */
}

fn takes_ownership(some_string: String) { // A String e passada para a funcao que se torna "proprietaria" para a mesma
   println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
 // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
   println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
