/*use std::fs::File;
use std::io::{Read, BufReader, BufRead};

fn read_entire_file() {
    let mut file = File::open("example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("File contents:\n{}", contents);
}

fn read_file_line_by_line() {
    let file = File::open("example.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}*/
use std::process::Command;
use std::io::{self, Read, Write};
use std::fs::File;

fn read_file_linux(filename: String) {
    let output = Command::new("cat")
        .arg(&filename)
        .output()
        .expect("Failed to execute command");

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
} 

fn reading_from_console(message: String) -> String {
    let mut buffer = String::new();

    print!("{}", &message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();
    return name;
}

fn confirmation_from_console() -> String {
    let mut buffer = String::new();

    print!("Do you want to make a new file (1), open an exsisting one (2), or finish the program (3)?");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();
    return name;
}
/*
fn create_file_linux(filename: String) {
    let output = Command::new("touch")
        .arg(&filename)
        .output()
        .expect("Failed to execute command");

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}

fn write_file_linux(text: String, file: String) {
    let output = Command::new("echo")
        .arg(&text.to_string())
        .arg(" >> ")
        .arg(&file)
        .output()
        .expect("Failed to execute command");

    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}
    */

fn create_and_write_to_file(filename: &String){
    let mut file = File::create(&filename).unwrap();
    writeln!(file, "Hello, Rust file opperations!").unwrap();
    writeln!(file, "This is a new line!").unwrap();

}

fn main() {
    /*
    println!("Reading entire file:");
    read_entire_file();

    println!("\nReading file line by line:");
    read_file_line_by_line();
    */

    //ask user if he wants to create and edit a new file or open an exsisting one
    //Creating: make a file and write inside it etirely from the console
    while true{
        let choice = confirmation_from_console().parse();

        match choice{
            Ok(1) => {/*
            let mut message = "What is your new file called?".to_string();
            let new_file = reading_from_console(message);
            create_file_linux(new_file.clone());
            message = "What do you want to write?".to_string();
            let text = reading_from_console(message);
            write_file_linux(text.to_string(), new_file);
            */
            let mut message = "What is your new file called?".to_string();
            let new_file = reading_from_console(message);
            create_and_write_to_file(&new_file);
            },
    //ask user what file to open
            Ok(2) => {
            let message = "What file do you want to open?".to_string();
            let filename = reading_from_console(message);
            read_file_linux(filename);
            },
            Ok(3) => break,
            _ => {}
        }
    }
    
}