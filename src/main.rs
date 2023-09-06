mod parser;
mod tokenizer;

use regex::Regex;
use std::io::Write;

fn compile(input: String) -> Result<String, String> {
    let _ast = parser::parse(input)?;
    todo!()
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let content: String;

    let mut destination_file: std::fs::File;

    let filename_regex = Regex::new(r"(\w*).c").unwrap();
    match filename_regex.captures(&args[1]) {
        Some(name) => {
            let filename = format!("{}.s", &name[1]);
            content = std::fs::read_to_string(&args[1]).expect(&format!("Couldn't open file!"));
            destination_file = std::fs::File::create(filename).unwrap();
        }
        None => {
            panic!("{}: not a C source file", &args[1]);
        }
    }

    destination_file
        .write_all(compile(content)?.as_bytes())
        .unwrap();

    Ok(())
}

