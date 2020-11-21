use std::collections::HashMap;

// no momento que linkamos a struct query string no modulo em mod.rs ele reclama do lifetime entao estamos definindo 
// em toda a estrutura do modulo 


#[derive(Debug)]
pub struct QueryString<'buffer>{
    data:HashMap<& 'buffer str, QueryStringValue<'buffer>>
}


// segue um exemplo de querystring - > a=1&b=2&c&d=&e===&d=7&d=abc
// precisamos tratar todos os casos para estabelecer uma equivalencia chave/ valor para a qs


#[derive(Debug)]
pub enum QueryStringValue<'buffer> {
    Single(& 'buffer str),
    Multiple( Vec<& 'buffer str>)
}

impl<'buffer> QueryString<'buffer> {

    fn get(&self,key: &str)->Option<&QueryStringValue>{
        self.data.get(key)
    }
} 

// como a trait FromStr nao permite a utilizacao de lifetimes customizados estamos usando 
// a trait From
impl<'buffer> From<&'buffer str> for QueryString<'buffer>
{
    fn from(string:& 'buffer str)->Self
    {
        let mut data=HashMap::new();
        
        
        for sub_string in string.split("&"){ // o split retorna um iterator 

            let mut key = sub_string;
            let mut value = "";
            if let Some(index) = sub_string.find("="){ // if let se encontrarmos um igual no slice
                key = &sub_string[..index]; // separando a key com o texto antes do igual
                value= &sub_string[index+1..]; // separando a key com o texto depois do igual  ( so da certo pq temos caracteres de ocupam 1 byte)


                // usando a funcao entry do hashmap para para recuperar o registro e alteracoes inplace
                // para fazer bruxaria bem interessantes 

                data.entry(key).and_modify(|reg: &mut QueryStringValue| //usando a sintaxe de clojure para alterar um registro que ja exite com .and_modify
                {
                    match reg{
                        QueryStringValue::Multiple(vector_of_values) =>{ // se ja tivermos um multiple estamos adicionando um valor
                            vector_of_values.push(value);
                        },
                        QueryStringValue::Single(current_val)=>{ // se a chave ja existir e for um valor Single deve ser transformado em um valor multiple e ter ambos os valores adicionados ao vetor
                            *reg = QueryStringValue::Multiple(vec![current_val,value]);

                        }

                    }
                }).or_insert(QueryStringValue::Single(value)) ; // com a funcao or_insert o registro e inserido se nao existir



            }
            

        }
        
        
        
        QueryString{data}
        
        
        
        
        
        
        
        //unimplemented!();

    }
}

