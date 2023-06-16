mod tokenizer;

use regex::Regex;
use std::io::Write;

fn compile(input: String) -> Result<String, String> {
    let code_regex = Regex::new(r"\s*int\s*main\s*\(\)\s*\{\s*return\s+(\d+)\s*;\s*\}\s*").unwrap();

    match code_regex.captures(&input) {
        Some(literal) => Ok(format!(
            r#"    .globl main
main:
    movl ${}, %eax
    ret
"#,
            &literal[1]
        )),
        None => Err("easy, tiger!".into()),
    }
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
            println!("{}", &args[1]);
            panic!("not a C source file");
        }
    }

    destination_file
        .write_all(compile(content)?.as_bytes())
        .unwrap();

    Ok(())
}

#[test]
fn level1_pure_regex() {
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

    assert_eq!(intended, compile(input).unwrap());
}

#[test]
fn level1_tokens() {
    let input = r#"int main() {
    return 2;
}"#;

    let received = tokenizer::tokenize(&input).unwrap();

    let intended = vec![tokenizer::TokenKind::Identifier("int".into()),
    tokenizer::TokenKind::Identifier("main".into()),
    tokenizer::TokenKind::LCurly,
    tokenizer::TokenKind::Identifier("return".into()),
    tokenizer::TokenKind::Integer(2),
    tokenizer::TokenKind::SemiColon,
    tokenizer::TokenKind::RCurly,
    ];

    assert_eq!(intended, received);
}
