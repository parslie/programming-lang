use std::{env, fs::OpenOptions, io::Read, path::Path};

mod token;

fn main() {
    // TODO properly implement argument parsing
    let args: Vec<String> = env::args().collect();
    let file_path = Path::new(args[1].as_str());
    let mut file = OpenOptions::new().read(true).open(file_path).unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    let tokens = token::tokenize(&file_content);
    println!("Tokens: {:?}", tokens);

    // TODO create abstract syntax tree
}
