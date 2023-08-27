use crate::tokenizer;

pub enum ASTNodeKind {
    Integer(usize),
}

struct Parser {
    // NOTE: maybe this becomes a slice like the tokenizer
    tokens: Vec<tokenizer::TokenKind>,
}

impl Parser {
    fn from_tokens(tokens: Vec<tokenizer::TokenKind>) -> Result<Parser, String> {
        Ok(Parser { tokens })
    }
}

pub fn parse_tokens(text: String) -> Result<ASTNodeKind, String> {
    let parser = Parser::from_tokens(tokenizer::tokenize(&text)?);
    todo!()
}
