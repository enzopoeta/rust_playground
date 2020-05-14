fn main() {
    
    // Vetores 
    // Declaracao normal de um vetor
    let _v: Vec<i32> = Vec::new();

    // Declaracao de um vetor usando a macro vec! ( o rust tenta fazer a inferencia de tipos)

    let _v = vec![1, 2, 3];

    // Adicionando elementos a um vetor (repare o mut)
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // como qq outra struct ainda se um vetor sai de escopo os dados são removidos da memoria
    {
        
        let _v = vec![1, 2, 3, 4];
        println!("contextual _v Content {:?}",_v);


        // do stuff with v
    } // <
    println!("original _v Content {:?}",_v);

    // Leitura de dados de vetores 
    // 2 jeitos basicos pelo indice e por get
    // Com indice (neste caso vc pode incorrer em um panic por acessar um indice inexistente)
    let mut _third: i32 = v[2]; // vc pode pegar o elemento por copia
    println!("third {}",_third);
    v[2]= 9 as i32;
    println!("third after change in vector {}",_third);
    let mut _third = &v[2]; // ou por referencia
    println!("third {}",_third);
    println!("third after change in vector {}",_third);

    
    // Com match e get voce obtem uma option com um some que tem o valor desejado caso o 
    // indice exista ou um Nome caso o elemento nao exista
    // o que eh mais seguro    
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // iterando sobre os elementos de um vetor
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // voce tambem por iterar por um vetor mutavel para alterar os valores do mesmo 
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;  // e preciso utilizar * (operador de dereferencia) para chegar ao valor efetivo da referencia
    }

    // apesar do vetor ser tipado eh possivel utilizar enumerations para armazenar dados heterogeneos
    // veja o exemplo abaixo
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("ROW -> {:?}",row);


    // Strings

    //Criando um objeto String atraves de um string literal
    let s = String::from("initial contents");









}
