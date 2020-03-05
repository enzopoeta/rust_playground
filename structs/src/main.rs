// definicao padrao de uma struct
#[derive(Debug)] //como as structs nao implementam a trait std::fmt::Display elas nao podem por default elas nao podem ser printadas, por isso adicionamos a trait derive(Debug) para que isso seja possivel 
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
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


}
