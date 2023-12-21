use super::tokenizer::{tokenize, TokenKind};
use super::{ASTExpression, ASTExpressionKind, ASTNode, ASTNodeKind, AST};

struct Parser {
    // NOTE: maybe this becomes a slice like the tokenizer
    tokens: Vec<TokenKind>,
    counter: usize,
}

impl Parser {
    fn from_tokens(tokens: Vec<TokenKind>) -> Result<Self, String> {
        Ok(Parser { tokens, counter: 0 })
    }

    fn from_input(input: String) -> Result<Self, String> {
        Ok(Parser {
            tokens: tokenize(input)?,
            counter: 0,
        })
    }

    fn peek(&self, offset: usize) -> Option<&TokenKind> {
        self.tokens.get(self.counter + offset)
    }

    fn current(&self) -> Option<&TokenKind> {
        self.peek(0)
    }

    fn next_node(&mut self) -> Option<ASTNode> {
        let ret = match self.current()? {
            TokenKind::Integer(i) => Some(ASTNode::from(ASTExpression::from(*i))),
            _ => todo!(),
        };
        self.counter += 1;

        ret
    }
}

pub fn parse(text: String) -> Result<AST, String> {
    let tokens = tokenize(text)?;
    let mut parser = Parser::from_tokens(tokens)?;
    let mut ast = AST::new();

    while let Some(node) = parser.next_node() {
        ast.nodes.push(node);
    }

    Ok(ast)
}
