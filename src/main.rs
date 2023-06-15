mod tokenizer;

use regex::Regex;
use std::io::Write;

fn compile(input: String) -> String {
    let code_regex = Regex::new(r"\s*int\s*main\s*\(\)\s*\{\s*return\s+(\d+)\s*;\s*\}\s*").unwrap();

    match code_regex.captures(&input) {
        Some(literal) => {
            format!(
                r#"    .globl main
main:
    movl ${}, %eax
    ret
"#,
                &literal[1]
            )
        }
        None => panic!("easy, tiger!"),
    }
}

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
        }
    }

    destination_file
        .write_all(compile(content).as_bytes())
        .unwrap();
}


#[test]
fn level1() {
    let input: String = r#"int main() {
    return 2;
}"#
    .into();

    let intended: String = r#"    .globl main
main:
    movl $2, %eax
    ret
"#
    .into();

    assert_eq!(intended, compile(input));
}
