mod tokenizer;

use std::io::Write;

use regex::Regex;

fn main() {
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
            println!("{}", &args[1]);
            panic!("not a C source file");
        },
    }
    let code_regex =
        Regex::new(r"\s*int\s*main\s*\(void\)\s*\{\s*return\s+(\d+)\s*;\s*\}\s*").unwrap();

    match code_regex.captures(&content) {
        Some(literal) => {
            destination_file
                .write_all(
                    format!(
                        r#"    .globl main
main:
    movl ${}, %eax
    ret"#,
                        &literal[1]
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
        None => println!("easy, tiger!"),
    };
}
