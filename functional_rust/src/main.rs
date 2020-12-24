fn main() {
    
    // teste de programacao funcional com rust

    // funcao generica comum 
    fn sum(val1:i32,val2:i32) -> i32 {
        val1+val2
    }


    // passando uma funcao como parametro 
    fn operation<F>(val1:i32,val2:i32,operation_function:F) -> i32 where F: Fn(i32,i32) -> i32{
            // o where serve para definir o tipo so parametro generico F
            operation_function(val1,val2)
        }
    
    // testando

    println!("Rodando a funcao operacao com os parametros 1 e 3 e passando a funcao soma como parametro - {}",operation(1,3,sum));


    // funcoes anonimas (closures)
    // sintaxe
    let _sqrt=|x:i32| x*x; // o valor entre || delimita os parametros e o resto delimita o corpo da funcao


    // pode ser necessario usar a sintaxe mais completa para a closure (as vezes o compilador nao consegue inferir tudo)
    let _sqrt_simples = |x:i32|->i32 {x*x}; // as chaves sao necessarias quando a funcao tem mais de uma linnha


    // uma outra caracteristica interessante das closures e que elas podem acessar dados do ambiente onde ela foi criada 
    // diretamente. Ex:

    const THE_NUMBER:i32=42i32;

    let sum_with_the_number = |x:i32| x+THE_NUMBER ;

    // testando 
    println!("Rodando a funcao sum_with_THE_NUMBER com o parametro 10 - {}",sum_with_the_number(10i32));

    // CLAUSULA MOVE 
    // para resolver problemas com borrowing de das variaveis no ambiente das closures existe a contrucao move que faz o "move"
    // de todo o contexto para a o escopo da closure
    #[derive(Debug)]
    struct User {uid:String,name:String};

    let mut user = User {uid:"10".to_string(),name:"Joaozinho".to_string()};

    // criando a closure sem o move
    let mut muda_nome = |new_name:String| user.name=new_name;
    
    // a struct User continua referenciada pela variavel user
    muda_nome("Enzo Telles Poeta".to_string());
    println!("struct ainda esta associada a variavel user {:?}",user);

    // agora com o move
    let mut muda_nome = move |new_name:String| user.name=new_name;
    muda_nome("Enzo Telles Poeta".to_string());
    // aqui como movemos o escopo a variavel user ja nao tem mais a propriedade da struct User entao o compilator vai reclamar
    //println!("struct ainda esta aqui {:?}",user); // compilador reclama do borrowing via move


    // Como retorna funcoes
    // o retorno de uma funcao e feito atraves do encapsulamento em um smartpointer Box (o box permite que voce aloque um valor na heap)
    // O box tem todos os controle normais de escopo do rust
    enum CharacterAction{
        Right,
        Left,
        Up,
        Down,
    }
    
    
    
    fn get_char_action_impl(char_action: CharacterAction)->Box<dyn Fn((i32,i32))->(i32,i32)> // como Fn e um objeto do tipo trair e preciso colocar o dyn na frente
    {
        let move_amplitude=5i32;
        
        // o move e necessario para podermos mover o escopo da variavel move_amplitude para fora do escopo da funcao get_char_action_impl
        match char_action {
            CharacterAction::Right => Box::new(move |coordinate_pair| {(coordinate_pair.0+move_amplitude,coordinate_pair.1)}),
            CharacterAction::Down => Box::new(move |coordinate_pair| {(coordinate_pair.0,coordinate_pair.1-move_amplitude)}),
            CharacterAction::Up => Box::new(move |coordinate_pair| {(coordinate_pair.0,coordinate_pair.1+move_amplitude)}),
            CharacterAction::Left => Box::new(move |coordinate_pair| {(coordinate_pair.0-move_amplitude,coordinate_pair.1)}),
        }
        
       
        //let num = 5;Box::new(move |x| x + num)
    }

    // testando
    
    let mut char_position = (0,0);
    println!("Posicao inicial do personagem : {:?}",char_position);
    
    let char_up_action = get_char_action_impl(CharacterAction::Up);
    char_position = char_up_action(char_position);
    
    println!("Nova posicao do personagem depois do char_up_action : {:?}",char_position);

    
    
    // ITERATORS / ADAPTERS / Consumers

    // para criar um iterator em uma estrutura que  implementa a trait  Iterator podemos utilizar
    // .iter() para obter um iterador por referencia e .into_iter() para obter um iterador com 
    // copia (move) dos valores da estrutura original
    // ex: 
    let frutas = vec!["banana","mamao","melao","abacate"];

    // passagem por referencia
    let numero_letras = frutas.iter().map(|fruta:&&str| fruta.len()).fold(0,|soma,len| soma+len);
    println!("numero de letras no vec : {}",numero_letras);

    // move do dado ( perde a ownership)
    let numero_letras = frutas.into_iter().map(|fruta:&str| fruta.len()).fold(0,|soma,len| soma+len);
    println!("numero de letras no vec : {}",numero_letras);
    //println!("numnero_letras :{:?}",frutas); descomentar isso gera um erro (frutas ja nao tem mais o ownership do dado)

    // MAPs, filters,  folds, etc...
    // os iterators possuem funcoes que podem ser utilizadas em conjunto para os mais variados fins... ai vai um resumo
    
    //.map() -- aplica uma closure a cada elemento do vetor e retorna o resultado para a proxima funcao aplicada :
    let vetor = vec![1,2,3,4,5];
    println!("O VETOR -- {:?}",vetor);
    // vamos usar o map para transformar todos os valores em valores pares
    let vetor_pares:Vec<i32> = vetor.iter().map(|&x| x*2).collect();
    println!("vetor pares : {:?}",vetor_pares);

    
    //filter() -- como o proprio nome sugere ele filtra os valores de acordo com a funcao closure passada para
    // filtrando todos os elementos que nao sao pares
    let vetor_impares:Vec<&i32> =vetor.iter().filter(|&x| x %2 !=0).collect();//.collect();
    println!("vetor impares : {:?}",vetor_impares);
    // como  uma pequena variacao podemos contar os numeros impares
    let qtd_impares = vetor.iter().filter(|&x| x %2 !=0).count();
    println!("numero de impares : {:?}",qtd_impares);

    //fold() -- usa uma closure para acumular de alguma forma os elementos do vetor em uma unica linha vetor
    let soma_elementos = vetor.iter().fold(0,|soma,x| soma+x);
    println!("soma dos elementos do vetor {}",soma_elementos);


    //zip() une 2 vetores
    let index_vec = vec![0,1,2,3,4];
    let vetor_tupla:Vec<(&i32,i32)> = vetor.iter().zip(index_vec).collect();
    println!("vetor de tuplas {:?}",vetor_tupla);

    // max() exibe o valor maximo de 1 vetor
    let max_value = vetor.iter().max().unwrap();
    println!("Valor maximo do vetor {:?}",max_value);


}
