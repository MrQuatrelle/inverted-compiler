use crate::tokenizer::{self, TokenKind};

pub enum ASTNodeKind {}

struct Parser<'a> {
    // NOTE: maybe this becomes a slice like the tokenizer
    tokens: &'a Vec<tokenizer::TokenKind>,
}

impl<'a> Parser<'a> {
    fn from_tokens(tokens: &Vec<tokenizer::TokenKind>) -> Result<Parser, String> {
        Ok(Parser { tokens })
    }

    fn parse(&mut self) -> Result<ASTNodeKind, String> {
        todo!()
    }
}

pub fn parse(text: String) -> Result<ASTNodeKind, String> {
    let parser = Parser::from_tokens(&tokenizer::tokenize(&text)?);
    todo!()
}
