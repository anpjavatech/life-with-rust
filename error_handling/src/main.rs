use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    let file_handler = File::open("test.txt");
    let mut file_content = match file_handler {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file : {error:?}"),
            },
            _ => panic!("problem opening the file: {error:?}"),
        },
    };
    let mut contents = String::new();
    match file_content.read_to_string(&mut contents) {
        Ok(_) => println!("Contents of the file : {contents:?}"),
        Err(error) => println!("error while reading the file : {error:?}"),
    }
}
