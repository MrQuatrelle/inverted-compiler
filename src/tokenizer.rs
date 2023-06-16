/// Kinds of tokens of the inverted-C lang
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    SemiColon,
    LParenthesis,
    RParenthesis,
    LCurly,
    RCurly,
    Identifier(String),
    Integer(usize),
}

impl From<String> for TokenKind {
    fn from(value: String) -> Self {
        TokenKind::Identifier(value)
    }
}

impl<'a> From<&'a str> for TokenKind {
    fn from(value: &'a str) -> Self {
        TokenKind::Identifier(value.to_string())
    }
}

impl From<usize> for TokenKind {
    fn from(value: usize) -> Self {
        TokenKind::Integer(value)
    }
}

fn tokenize_single_integer(slice: &str) -> Result<(TokenKind, usize), String> {
    let mut offset = 0_usize;
    let mut iter = slice.chars();

    loop {
        match iter.next() {
            Some(c) => {
                if c.is_numeric() {
                    offset += 1;
                }
            }
            None => {
                break;
            }
        };
    }

    if offset == 0 {
        Err("Not an integer".into())
    } else {
        let buffer = &slice[..offset];
        match buffer.parse::<usize>() {
            Ok(i) => Ok((TokenKind::Integer(i), offset)),
            Err(_) => Err("Not an integer".into()),
        }
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

    fn next_token(&mut self) -> Result<Option<TokenKind>, String> {
        if self.remaining_content.is_empty() {
            Ok(None)
        } else {
            let next_char = match self.remaining_content.chars().next() {
                Some(c) => c,
                None => return Err("unexpected end of content".into()),
            };

            let (ret, offset) = match next_char {
                ';' => (TokenKind::SemiColon, 1),
                '(' => (TokenKind::LParenthesis, 1),
                ')' => (TokenKind::RParenthesis, 1),
                '{' => (TokenKind::LCurly, 1),
                '}' => (TokenKind::RCurly, 1),
                '0'..='9' => tokenize_single_integer(self.remaining_content)?,
                _ => return Err("Unsupported pattern".into()),
            };
            self.remaining_content = &self.remaining_content[offset..];
            Ok(Some(ret))
        }
    }
}

pub fn tokenize(content: &str) -> Result<Vec<TokenKind>, String> {
    let mut tokenizer = Tokenizer::new(content);
    let mut tokens: Vec<TokenKind> = Vec::new();

    loop {
        match tokenizer.next_token()? {
            Some(t) => tokens.push(t),
            None => break,
        };
    }

    Ok(tokens)
}

#[test]
fn parse_symbols_test() {
    let mut tokenizer = Tokenizer::new(";(){}1234");
    assert_eq!(Some(TokenKind::SemiColon), tokenizer.next_token().unwrap());
    assert_eq!(Some(TokenKind::LParenthesis), tokenizer.next_token().unwrap());
    assert_eq!(Some(TokenKind::RParenthesis), tokenizer.next_token().unwrap());
    assert_eq!(Some(TokenKind::LCurly), tokenizer.next_token().unwrap());
    assert_eq!(Some(TokenKind::RCurly), tokenizer.next_token().unwrap());
    assert_eq!(Some(TokenKind::Integer(1234)), tokenizer.next_token().unwrap());
    assert_eq!(None, tokenizer.next_token().unwrap());
}
