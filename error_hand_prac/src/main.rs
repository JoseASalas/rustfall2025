
//fn div(a,b){
//    1) panic
//    2) return error

//    return a/b;
//}

use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;


enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    et mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)? //? are supposed to represent match statements
    Ok(s)
}

fn main() {
    read_username_from_file();
}
