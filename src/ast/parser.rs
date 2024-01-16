use super::tokenizer::{tokenize, TokenKind};
use super::{ASTBinaryExpression, ASTExpression, ASTExpressionKind, ASTNode, ASTNodeKind, AST};

struct Parser {
    // NOTE: maybe this becomes a slice like the tokenizer
    tokens: Vec<TokenKind>,
    ast: AST,
    counter: usize,
}

impl Parser {
    fn peek(&self, offset: isize) -> Option<&TokenKind> {
        self.tokens.get((self.counter as isize + offset) as usize)
    }

    fn current(&self) -> Option<&TokenKind> {
        self.peek(0)
    }

    fn extract_args_binary(&mut self) -> Option<(ASTExpression, ASTExpression)> {
        let left: ASTExpression = match self.ast.pop_last() {
            Some(node) => node.try_into().expect("not an expression"),
            None => panic!("parse error: no node to the left of binary operator"),
        };

        match left.kind {
            ASTExpressionKind::IntegerLiteral(_) | ASTExpressionKind::Binary(_) => {}

            // NOTE: remove on final build
            #[allow(unreachable_patterns)]
            _ => panic!("parse error: node to the left of binary operator isn't valid/supported"),
        }

        self.counter += 1;

        let right = match self.current() {
            Some(token) => token,
            None => panic!("parse error: no node to the right of binary operator"),
        };

        let right: ASTExpression = match right.try_into() {
            Ok(exp) => exp,
            Err(_) => {
                panic!("parse error: right side of the binary operator is not a valid expression")
            }
        };

        Some((left, right))
    }

    fn parse_binary(&mut self) -> Option<()> {
        let (left, right) = self.extract_args_binary()?;

        self.ast.push(ASTNode::from(ASTExpression {
            kind: ASTExpressionKind::Binary(ASTBinaryExpression::new(
                match self.peek(-1).expect("I've been there before...") {
                    TokenKind::Plus => super::BinaryOperatorKind::Plus,
                    TokenKind::Hyphen => super::BinaryOperatorKind::Minus,
                    TokenKind::Asterisk => super::BinaryOperatorKind::Mult,
                    TokenKind::Slash => super::BinaryOperatorKind::Div,
                    _ => todo!(),
                },
                left.into(),
                right.into(),
            )),
        }));

        Some(())
    }

    fn parse(mut self) -> Option<AST> {
        while self.counter < self.tokens.len() {
            let token = self.current()?;
            match token {
                TokenKind::Integer(i) => {
                    self.ast.push(ASTNode::from(ASTExpression::from(*i)));
                }

                TokenKind::Plus | TokenKind::Hyphen | TokenKind::Asterisk | TokenKind::Slash => {
                    self.parse_binary();
                }

                _ => panic!("[TODO]: token no supported by the parser"),
            }

            self.counter += 1;
        }

        Some(self.ast)
    }
}

impl From<Vec<TokenKind>> for Parser {
    fn from(tokens: Vec<TokenKind>) -> Self {
        Self {
            tokens,
            counter: 0,
            ast: AST::new(),
        }
    }
}

impl From<String> for Parser {
    fn from(value: String) -> Self {
        match tokenize(value) {
            Some(tokens) => Self {
                tokens,
                counter: 0,
                ast: AST::new(),
            },
            None => panic!("tokenizer failed while trying to create parser"),
        }
    }
}

pub fn parse(text: String) -> Option<AST> {
    let tokens = tokenize(text)?;
    let parser = Parser::from(tokens);

    parser.parse()
}
