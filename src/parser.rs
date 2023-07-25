use std::cell::RefCell;
use std::rc::Rc;

use crate::tokenizer::TokenKind;
use crate::tokenizer::VarType;

enum ParserNodeType {
    Program,
    Function(VarType),
    Return(VarType),
    LiteralInt(usize),
}

pub struct ParserNode {
    node_type: ParserNodeType,
    next: Option<Rc<RefCell<ParserNode>>>,
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

    fn build_ast(&self) -> Result<Rc<RefCell<ParserNode>>, String> {
        let mut iter = self.remaining_tokens.iter().peekable();
        let head = Rc::new(RefCell::new(ParserNode {
            node_type: ParserNodeType::Program,
            next: None,
        }));
        let mut tmp = head.clone();
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
                let buffer2 = Some(Rc::new(RefCell::new(buffer)));
                tmp.borrow_mut().next = buffer2.clone();
                tmp = buffer2.clone().unwrap();
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
                if let TokenKind::Integer(i) = next {
                    let buffer = ParserNode {
                        node_type: ParserNodeType::Return(VarType::Int),
                        next: Some(Rc::new(RefCell::new(ParserNode {
                            node_type: ParserNodeType::LiteralInt(*i),
                            next: None,
                        }))),
                    };
                    let buffer2 = Some(Rc::new(RefCell::new(buffer)));
                    tmp.borrow_mut().next = buffer2.clone();
                    tmp = buffer2.clone().unwrap();
                };
            }
            _ => return Err("only main with a return statement supported yet".into()),
        }

        match iter.next() {
            Some(TokenKind::RCurly) => {}
            _ => return Err("missing closing curly brace".into()),
        }

        Ok(head.clone())
    }
}

pub fn parse(tokens: Vec<TokenKind>) -> Result<Rc<RefCell<ParserNode>>, String> {
    let parser = Parser::new(tokens);
    todo!()
}
