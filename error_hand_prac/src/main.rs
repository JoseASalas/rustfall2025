
//fn div(a,b){
//    1) panic
//    2) return error

//    return a/b;
//}

//use std::fs::File;
//use std::io::ErrorKind;
//use std::io::Read;


//enum Result<T, E> {
//    Ok(T),
//    Err(E),
// }

fn serde_serialize() {
    use serde::{Serialize, Deserialize};
    #[derive(Deserialize)]
    #[derive(Serialize)]
    struct Person {
        name: String,
        age: u8,
    }
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
    };
    let serialized = serde_json::to_string(&person).unwrap();

    println!("Serialized Person = {:?}", serialized);
    let person:Person = serde_json::from_str::<Person>(serialized.as_str()).unwrap();

    println!("Deserialized Person = {}, {}", person.name, person.age);
}

fn serde_deserialize(){
    use serde::{Serialize, Deserialize};
    #[derive(Deserialize)]
    struct Person {
        name: String,
        age: u8,
    }
    let data = r#"{"name": "John Doe", "age": 30}"#;
    let person: Person = serde_json::from_str::<Person>(data).unwrap();
    println!("Deserialized Person = {}, {}", person.name, person.age);
}

fn main() {
    serde_serialize();
    serde_deserialize();
}
