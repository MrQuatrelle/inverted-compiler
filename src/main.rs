pub mod ast;
use regex::Regex;
use std::{io::Write, process::exit};

macro_rules! print_help {
    () => {
        print!(
            r#"
Inverted C compiler by MrQuatrelle

    usage: icc [ARGS] <source files>

    ARGS:
            (TODO)
            ...
"#
        );
    };
}

fn compile(input: String) -> Result<String, String> {
    let ast = ast::parser::parse(input)?;
    ast.visualize();

    Ok("".into())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let content: String;

    if args.len() < 2 {
        print_help!();
        exit(1);
    }

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
