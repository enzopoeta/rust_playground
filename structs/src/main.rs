// definicao padrao de uma struct
#[derive(Debug)] //como as structs nao implementam a trait std::fmt::Display elas nao podem por default elas nao podem ser printadas, por isso adicionamos a trait derive(Debug) para que isso seja possivel 
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/// também é possiível adicionar funcões que operam nas instancias de uma structs 
/// isso é feito com a declaracao impl congforme o exemplo abaixo
/// o primeiro parametro de funcao sempre deve ser a referencia a propria struct (&self)
impl User {
    fn get_email_info(&self) -> String {
        self.email.clone()
    }
}



// se temos uma funcao que utiliza parametros com nomes iguais aos atributos da sequence nao precisamos seta-los especificamente ao 
// fazer a atribuicao como podemos verificar no exemplo abaixo
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {



    
    //declaracao de uma struct especifica
    let mut user1 = User {
        email: String::from("enzopoeta@gmail.com"),
        username: String::from("enzopoeta"),
        active: true,
        sign_in_count: 1,
    };

    println!("mostrando a struct completa {:?}\n",user1);

    // para acessar um valor especifico de da struct podemos usar a sintaxe de ponto
    println!("acessando o email do usuario {}\n",user1.email);

    // tambem e possivel utilizar a sintaxe de ponto para setar um dos valores de uma struct (desde que ela seja mutavel)
    user1.email = String::from("enzotp@unicamp.br");
    println!("acessando o email do usuario apos alteracao {}\n",user1.email);



    // tambem é possivel copiar os valores de uma struct utilizando a seguinte sintaxe
    let user2 = User {
        email: String::from("another@example.com"), // email e username sao campos que desejamos colocar valores especificos
        username: String::from("anotherusername567"),
        ..user1 // o resto sera copiado do user 2
    };

    println!("printando o usuario 2 --> {:?}",user2);

    
    // existe tambem a possibilidade de utilizarmos a construção de structs sem nomes de atributos definidos (tuplas tipadas)
    // conformme os exemplos abaixo
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("printando A COR PRETA --> {:?}",black);

    println!("printando o ponto origem --> {:?}",origin);

    // e possivel inclusive criar uma tupla sem quaisquer campos ! isso pode ser util quando vc precisar criar uma trait por exemplo 
    // (vc precisa criar um tipo novo e nao precisa passar nenhum atributo p ele)

    /*
            #-- Structs e ownership  --#

            Em principio nao eh possivel passar valores de struct por referencia , porque pode ser que em algum momento o referido dado saia de escopo
            é possivel fazer isso se for definido um lifetime para o mesmo  (esse processo sera estudado mais a frente)
    */
    
    /// chamando metodos de uma struct 
    println!("printando o email pelo metodo de instancia da struct User -> {}",user1.get_email_info());
}