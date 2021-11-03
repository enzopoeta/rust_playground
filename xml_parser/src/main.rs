use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::str;

fn read_file(str_file_path: &str) -> Result<BufReader<File>, io::Error> {
    let PATH_PREFIX = "/data/rust_playground/xml_parser/dict/";

    let mut complete_path = PathBuf::new();

    complete_path.push(&PATH_PREFIX);
    let mut file_path = Path::new(&str_file_path);
    if file_path.has_root() {
        file_path = file_path.strip_prefix("/").unwrap();
    }

    complete_path.push(file_path);

    println!("Complete path ==> {}", complete_path.display().to_string());

    // verificando se o cara esta tentando explorar problemas de path traversal

    match complete_path.canonicalize() {
        Ok(canonical_path) => {
            if !canonical_path.starts_with(&PATH_PREFIX) {
                //#println!("Tentativa de path traversal");
                panic!("Tentativa de path traversal");
            }
        }
        Err(e) => {
            return Err(e);
        }
    }

    //fs::read_to_string(complete_path).ok() // o .ok() converte a saida de uma funcao que retorna Result<T> para Option<T>
    if complete_path.is_file() {
        let f = File::open(complete_path);
        let mut f = match f {
            Ok(file) => {
                println!("arquivo lido com sucesso");
                file
            }
            Err(e) => return Err(e),
        };
        let reader = BufReader::new(f);
        Ok(reader)
    } else {
        panic!("invalid path (not a file) ");
    }
}

fn main() {
    //let file_path: &str = "file.xml";

    let xml = r#"<tag1 att1 = "test">
                <tag2><!--Test comment-->Test</tag2>
                <tag2>
                    Test 2
                </tag2>
            </tag1>"#;

    let file_reader = read_file("test.xml");
    match file_reader {
        Ok(f_reader) => {
            let mut reader = Reader::from_reader(f_reader);
            reader.trim_text(true);

            //let mut count = 0;
            //let mut txt = Vec::new();
            let mut buf = Vec::new();

            // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(ref e)) => match e.name() {
                        b"content" => {
                            let attrs: HashMap<String, String> = e
                                .attributes() // quick_xml::events::attributes::Attributes
                                .map(|a| {
                                    let attr = a.unwrap();

                                    // println!("{:?}", str::from_utf8(attr.key).unwrap());
                                    // println!("{:?}", std::str::from_utf8(attr.value.as_ref()));
                                    (
                                        str::from_utf8(&attr.key).unwrap().to_string(),
                                        str::from_utf8(&attr.value.as_ref()).unwrap().to_string(),
                                    )
                                })
                                .collect();
                            println!("{:?}", attrs);
                            //()
                        }
                        _ => (),
                    },
                    /*
                    Ok(Event::Text(e)) => {
                        println!("{:?}", e);
                        ()
                        //.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                    }*/
                    //Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
                    Ok(Event::Eof) => break, // exits the loop when reaching end of file
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    _ => (), // There are several other `Event`s we do not consider here
                }

                // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
                buf.clear();
            }
            /*
            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(ref e)) => count += 1,
                    Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).expect("Error!")),
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    Ok(Event::Eof) => break,
                    _ => (),
                }
                buf.clear();
            }
            println!("Found {} start events", count);
            println!("Text events: {:?}", txt);*/
        }
        Err(e) => println!("erro {}", e),
    }
}
