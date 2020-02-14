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
         voce esta cedendo a propriedade para a funcao ... se a funcao nao retornar o valor sai de escopo e é perdido
   */

   let x = String::from("Uma String");
   takes_ownership(x);
   //println!("{}",x); // ao decomentar essa linha veremos que nao eh mais possivel acessar o valor de x
   /*
      quando uma variavel sai de escopo o rust chama uma funcao especial chamada "drop" que implementa o procedimento 
      de liberacao de memoria -- e funcao do implementador personalizar o drop das suas libs 
   */

   let y = 156;
   makes_copy(y);
   println!("{}",y); // com variaveis da stack como numeros a passagem e feita por copia entao a variavel continua "usavel"


   /*
      Por uma questao de praticidade no rust voce pode utilizar uma referencia para "emprestar" uma variavel para ser utilizada
      em outros escopos / estruturas sem perder a propriedade da mesma
   */




}

fn takes_ownership(some_string: String) { // A String e passada para a funcao que se torna "proprietaria" da mesma
   println!("{}", some_string);
} // Ao sair da funcao a variavel perde o escopo e a memoria e liberada
 

fn makes_copy(some_integer: i32) { // Como no caso de integers a passagem do parametro e feita por copia a variavel continua usavel
   println!("{}", some_integer);
} // ai sair do escopo o integer tambem removido da memoria



