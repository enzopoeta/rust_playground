/*
    
O Rust possui vários recursos que permitem gerenciar a organização do seu código, incluindo quais detalhes são expostos, 
quais são privados e quais nomes estão em cada escopo de seus programas. Esses recursos, às vezes chamados coletivamente de sistema de módulos, incluem:

Packages: um recurso de cargo que permite criar, testar e compartilhar crates
Crates: Uma árvore de módulos que produz uma biblioteca ou executável
Módulos e use: permite controlar a organização, escopo e privacidade dos paths
Paths: Uma maneira de nomear um item, como uma estrutura, função ou módulo


*/



mod sample_module { // esta é a declaracao basica de um modulo

    fn connect(){ // a declaracao de funcoes e feita dentro do modulo

    }

    mod another_sample_module{ // tam bem e possivel tem um modulo aninhado dentro de outro modulo
        fn test_1()
        {

        }
        

        fn connect2(){ 
            // quando vc esta dentro da crate existem diversar maneiras de se invocar uma funcao
            // Ex1: usando caminho absoluto com crate
            crate::sample_module::connect();
            // Ex2: usando caminhos relativos com super (equivale a um ../)
            super::connect();


        }   
    }


}

pub mod ipv4; //quando o modulo e declarado desta maneira significa que sua implementacao esta em um arquivo separado. em determinadas situacoes 
// ( modulos dentro de modulos em arquivos separados ) pode ser necessária a criacao de pastas para conter os arquivos onde o nome da
//pasta e o nome do modulo e o arquivo rs principal recebe o nome mod.rs


/**
 
     ***structs e enums no sistema de modulos

     Também podemos usar o pub para designar structs e enums como públicas, 
     mas existem alguns detalhes extras. Se usarmos pub antes de uma definição de struct,
     tornaremos a struct pública, mas os campos da estrutura ainda serão privados. 
     Podemos tornar cada campo público ou não, caso a caso. Na Listagem 7-9, 
     definimos uma struct pública back_of_house :: Breakfast com um campo público toast,
     mas um campo seasonal_fruit privado. Isso modela o caso em um restaurante onde o cliente
    pode escolher o tipo de pão que acompanha uma refeição, mas o chef decide quais frutas acompanham
    a refeição com base no que está na estação e no estoque. A fruta disponível muda rapidamente, para 
     que os clientes não possam escolher a fruta ou até ver qual fruta obterão.


     Como o campo toast na struct back_of_house :: Breakfast é público, 
     podemos escrever e ler no campo toast usando a notação de ponto. 
     Observe que não podemos usar o campo seasonal_fruit no eat_at_restaurant 
     porque o seasonal_fruit é privado. Observe que, como back_of_house :: Breakfast possui 
     um campo privado, a struct precisa fornecer uma função pública associada que 
     constrói uma instância de Breakfast. Se  Breakfast não tivesse essa função, 
     não poderíamos criar uma instância de Breakfast porque não poderíamos definir o 
     valor do campo privado de seasonal_fruit.

 */

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

/*
    para as enums a regra eh mais simples :
    uma vez declarada publica todas as suas variacoes sao publicas

*/


