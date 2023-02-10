use std::env;
use std::fs;

mod error;
mod lexer;
mod token;

fn main() {
    let src_file: String = env::args().nth(1).unwrap();
    println!("File: {}", src_file);

    let data: Vec<u8> = fs::read(src_file).expect("Unable to read file");
    println!("{}", data.len());

    error::ErrorHandler().error(32, "help");
    
}
