/// Kinds of tokens of the inverted-C lang
enum TokenKind {
    Identifier(String),
    Integer(usize),
    LParenthesis,
    RParenthesis,
    LCurly,
    RCurly,
    SemiColon,
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
