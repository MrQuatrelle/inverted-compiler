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

fn tokenize_integer(slice: &str) -> Result<(TokenKind, usize), String> {
    let mut offset = 0_usize;

    for c in slice.chars() {
        if c.is_digit(10) {
            offset += 1;
        } else {
            break;
        }
    }

    if offset == 0 {
        Err(format!("Not an integer: '{}'", slice))
    } else {
        let buffer = &slice[..offset];

        if let Ok(i) = buffer.parse::<i64>() {
            Ok((TokenKind::Integer(i), offset))
        } else {
            Err(format!("Not an integer: '{}'", slice))
        }
    }
}

fn tokenize_string(slice: &str) -> Result<(TokenKind, usize), String> {
    let mut offset = 0_usize;

    if let Some(c) = slice.chars().next() {
        if c.is_digit(10) {
            return Err("unexpected digit in front of given slice".into());
        }
    }

    for c in slice.chars() {
        if !c.is_alphanumeric() {
            break;
        }

        offset += 1;
    }

    if offset == 0 {
        Err("Not an identifier".into())
    } else {
        let buffer = &slice[..offset];
        if buffer == "int" {
            Ok((TokenKind::Type(VarType::Int), offset))
        } else if buffer == "return" {
            Ok((TokenKind::Return, offset))
        } else {
            Ok((TokenKind::Identifier(buffer.into()), offset))
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

    fn next_token(&mut self) -> Result<Option<TokenKind>, String> {
        self._skip_whitespaces();

        if self.remaining_content.is_empty() {
            return Ok(None);
        }

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
            '0'..='9' => tokenize_integer(self.remaining_content)?,
            'A'..='Z' | 'a'..='z' | '_' => tokenize_string(self.remaining_content)?,
            _ => {
                return Err(format!(
                    "Unsupported pattern at: {}",
                    &self.remaining_content
                ))
            }
        };
        self.remaining_content = &self.remaining_content[offset..];
        Ok(Some(ret))
    }
}

pub fn tokenize(content: String) -> Result<Vec<TokenKind>, String> {
    let mut tokenizer = Tokenizer::new(&content);
    let mut tokens: Vec<TokenKind> = Vec::new();

    while let Some(t) = tokenizer.next_token()? {
        tokens.push(t)
    }

    Ok(tokens)
}

#[test]
fn tokenizer_test1() {
    let mut tokenizer = Tokenizer::new(";(){}1234");
    assert_eq!(Some(TokenKind::SemiColon), tokenizer.next_token().unwrap());
    assert_eq!(
        Some(TokenKind::LParenthesis),
        tokenizer.next_token().unwrap()
    );
    assert_eq!(
        Some(TokenKind::RParenthesis),
        tokenizer.next_token().unwrap()
    );
    assert_eq!(Some(TokenKind::LCurly), tokenizer.next_token().unwrap());
    assert_eq!(Some(TokenKind::RCurly), tokenizer.next_token().unwrap());
    assert_eq!(
        Some(TokenKind::Integer(1234)),
        tokenizer.next_token().unwrap()
    );
    assert_eq!(None, tokenizer.next_token().unwrap());
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