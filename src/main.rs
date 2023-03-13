use std::fs;

mod error;
mod lexer;
mod token;

fn main() {
    // let src_file: String = env::args().nth(1).unwrap();
    // println!("File: {}", src_file);

    // let data: Vec<u8> = fs::read(src_file).expect("Unable to read file");
    // println!("{}", data.len());
    let data: Vec<u8> = fs::read("C:\\code\\Personal Github\\go-compiler\\src\\test_files\\implements.go").expect("Unable to read file");
    for token in lexer::Lexer::new(&data) {
        println!("{}", token);
    };

    let data: Vec<u8> = fs::read("C:\\code\\Personal Github\\go-compiler\\src\\test_files\\hugeparams.go").expect("Unable to read file");
    for token in lexer::Lexer::new(&data) {
        println!("{}", token);
    };
    
}
