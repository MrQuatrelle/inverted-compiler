use crate::tokenizer::TokenKind;
use crate::tokenizer::VarType;

enum ParserNodeType {
    Program,
    Function(VarType),
    Return(VarType),
}

struct ParserNode {
    node_type: ParserNodeType,
    next: Option<Box<ParserNode>>,
}

struct Parser {
    remaining_tokens: Vec<TokenKind>,
}

impl Parser {
    fn new(tokens: Vec<TokenKind>) -> Self {
        Parser {
            remaining_tokens: tokens,
        }
    }

    fn build_ast(&self) -> Result<(), String> {
        let mut iter = self.remaining_tokens.iter().peekable();
        let mut head = ParserNode {
            node_type: ParserNodeType::Program,
            next: None,
        };
        let mut tmp: &ParserNode;
        match iter.next() {
            Some(TokenKind::Type(VarType::Int)) => {}
            _ => return Err("only lvl1 forms accepted".into()),
        }

        match iter.next() {
            Some(TokenKind::Identifier(name)) => {
                if name != &String::from("main") {
                    return Err("only function must be called main".into());
                }
                let buffer = ParserNode {
                    node_type: ParserNodeType::Function(VarType::Int),
                    next: None,
                };
                tmp = &buffer;
                head.next = Some(Box::new(buffer));
            }
            _ => return Err("only lvl1 forms accepted".into()),
        }

        match iter.next() {
            Some(TokenKind::LParenthesis) => {}
            _ => return Err("function declaration must be followed by argument tuple".into()),
        }

        match iter.next() {
            Some(TokenKind::RParenthesis) => {}
            _ => return Err("function declaration must be followed by argument tuple".into()),
        }

        match iter.next() {
            Some(TokenKind::LCurly) => {}
            _ => return Err("function declaration must be followed by argument tuple".into()),
        }

        match iter.next() {
            Some(TokenKind::Return) => {
                let next = iter.next().unwrap();
                let buffer = ParserNode {
                    node_type: ParserNodeType::Return(next)
                }
            },
            _ => return Err("only main with a return statement supported yet".into()),
        }






        match iter.next() {
            Some(TokenKind::RCurly) => {}
            _ => return Err("function declaration must be followed by argument tuple".into()),
        }
        Ok(())
    }
}
