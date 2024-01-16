use std::i64;

#[derive(Debug, PartialEq)]
pub enum VarType {
    Int,
}

/// Kinds of tokens of the inverted-C lang
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    SemiColon,
    LParenthesis,
    RParenthesis,
    LCurly,
    RCurly,
    Identifier(String),
    Integer(i64),
    Type(VarType),
    Return,
    Plus,
    Hyphen,
    Asterisk,
    Slash,
}

/// Implementations of `From` trait for `TokenKind`
impl From<String> for TokenKind {
    fn from(value: String) -> Self {
        TokenKind::Identifier(value)
    }
}

impl<'a> From<&'a str> for TokenKind {
    fn from(value: &'a str) -> Self {
        TokenKind::Identifier(value.into())
    }
}

impl From<i64> for TokenKind {
    fn from(value: i64) -> Self {
        TokenKind::Integer(value)
    }
}

fn tokenize_integer(slice: &str) -> Option<(TokenKind, usize)> {
    let mut offset = 0_usize;

    for c in slice.chars() {
        if c.is_digit(10) {
            offset += 1;
        } else {
            break;
        }
    }

    if offset == 0 {
        println!("Not an integer: '{}'", slice);
        return None;
    } else {
        let buffer = &slice[..offset];

        if let Ok(i) = buffer.parse::<i64>() {
            Some((TokenKind::Integer(i), offset))
        } else {
            println!("Not an integer: '{}'", slice);
            return None;
        }
    }
}

fn tokenize_string(slice: &str) -> Option<(TokenKind, usize)> {
    let mut offset = 0_usize;

    if let Some(c) = slice.chars().next() {
        if c.is_digit(10) {
            println!("unexpected digit in front of given slice");
            return None;
        }
    }

    for c in slice.chars() {
        if !c.is_alphanumeric() {
            break;
        }

        offset += 1;
    }

    if offset == 0 {
        println!("Not an identifier");
        None
    } else {
        Some(match &slice[..offset] {
            "int" => (TokenKind::Type(VarType::Int), offset),
            "return" => (TokenKind::Return, offset),
            id => (TokenKind::Identifier(id.into()), offset),
        })
    }
}

struct Tokenizer<'a> {
    remaining_content: &'a str,
}

/// Tokenizer for the C-lang inverted-compiler
impl<'a> Tokenizer<'a> {
    fn new(remaining_content: &'a str) -> Self {
        Self { remaining_content }
    }

    fn _skip_whitespaces(&mut self) {
        let mut offset = 0_usize;
        for c in self.remaining_content.chars() {
            if c.is_whitespace() {
                offset += 1;
            } else {
                break;
            }
        }

        self.remaining_content = &self.remaining_content[offset..];
    }

    fn next_token(&mut self) -> Option<TokenKind> {
        self._skip_whitespaces();

        if self.remaining_content.is_empty() {
            return None;
        }

        let next_char = self.remaining_content.chars().next()?;

        let (ret, offset) = match next_char {
            ';' => (TokenKind::SemiColon, 1),
            '(' => (TokenKind::LParenthesis, 1),
            ')' => (TokenKind::RParenthesis, 1),
            '{' => (TokenKind::LCurly, 1),
            '}' => (TokenKind::RCurly, 1),
            '0'..='9' => tokenize_integer(self.remaining_content)?,
            'A'..='Z' | 'a'..='z' | '_' => tokenize_string(self.remaining_content)?,
            '+' => (TokenKind::Plus, 1),
            '-' => (TokenKind::Hyphen, 1),
            '*' => (TokenKind::Asterisk, 1),
            '/' => (TokenKind::Slash, 1),
            _ => {
                println!("Unsupported pattern at: {}", &self.remaining_content);
                return None;
            }
        };
        self.remaining_content = &self.remaining_content[offset..];
        Some(ret)
    }
}

pub fn tokenize(content: String) -> Option<Vec<TokenKind>> {
    let mut tokenizer = Tokenizer::new(&content);
    let mut tokens: Vec<TokenKind> = Vec::new();

    while let Some(t) = tokenizer.next_token() {
        tokens.push(t)
    }

    Some(tokens)
}

#[test]
fn tokenizer_test1() {
    let mut tokenizer = Tokenizer::new(";(){}1234");
    assert_eq!(Some(TokenKind::SemiColon), tokenizer.next_token());
    assert_eq!(Some(TokenKind::LParenthesis), tokenizer.next_token());
    assert_eq!(Some(TokenKind::RParenthesis), tokenizer.next_token());
    assert_eq!(Some(TokenKind::LCurly), tokenizer.next_token());
    assert_eq!(Some(TokenKind::RCurly), tokenizer.next_token());
    assert_eq!(Some(TokenKind::Integer(1234)), tokenizer.next_token());
    assert_eq!(None, tokenizer.next_token());
}

#[test]
fn tokenizer_test2() {
    let input = r#"int main() {
    return 2;
}"#
    .into();

    let received = tokenize(input).unwrap();

    let intended = vec![
        TokenKind::Type(VarType::Int),
        TokenKind::Identifier("main".into()),
        TokenKind::LParenthesis,
        TokenKind::RParenthesis,
        TokenKind::LCurly,
        TokenKind::Return,
        TokenKind::Integer(2),
        TokenKind::SemiColon,
        TokenKind::RCurly,
    ];

    assert_eq!(intended, received);
}
