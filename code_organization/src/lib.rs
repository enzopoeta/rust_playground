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
        fn connect2(){ 

        }   
    }


}

pub mod ipv4; //quando o modulo e declarado desta maneira significa que sua implementacao esta em um arquivo separado. em determinadas situacoes 
// ( modulos dentro de modulos em arquivos separados ) pode ser necessária a criacao de pastas para conter os arquivos onde o nome da
//pasta e o nome do modulo e o arquivo rs principal recebe o nome mod.rs