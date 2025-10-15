use std::io::{self, Read, Write};
use std::fs::File;
use std::io::prelude::*;

struct Person {
    name: String,
    age: u32
}

struct Config{
    username: String,
    api_key: String,
    port: u16
}

impl COnfig{
    fn form_file(path: &str) -> Config{
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
    }
}

fn main() {
    reading_from_console();
    reading_from_file();
}