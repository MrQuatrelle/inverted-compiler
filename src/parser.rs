use crate::tokenizer::tokenize;
use crate::tokenizer::TokenKind;

pub enum ASTNodeKind {
    Integer(usize),
}

struct Parser {
    // NOTE: maybe this becomes a slice like the tokenizer
    tokens: Vec<TokenKind>,
}

impl Parser {
    fn from_tokens(tokens: Vec<TokenKind>) -> Result<Parser, String> {
        Ok(Parser { tokens })
    }
}

pub fn parse_tokens(text: String) -> Result<ASTNodeKind, String> {
    let parser = Parser::from_tokens(tokenize(&text)?);
    todo!()
}
