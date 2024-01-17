use super::tokenizer::{tokenize, TokenKind};
use super::{ASTBinaryExpression, ASTExpression, ASTExpressionKind, ASTNode, ASTNodeKind, AST};

struct Parser {
    // NOTE: maybe this becomes a slice like the tokenizer
    node_buffer: Option<ASTNode>,
    tokens: Vec<TokenKind>,
    counter: usize,
    ast: AST,
}

impl Parser {
    fn peek(&self, offset: isize) -> Option<&TokenKind> {
        self.tokens.get((self.counter as isize + offset) as usize)
    }

    fn current(&self) -> Option<&TokenKind> {
        self.peek(0)
    }

    fn extract_args_binary(&mut self) -> Option<(ASTExpression, ASTExpression)> {
        let left: ASTExpression = match match self.node_buffer.take() {
            Some(node) => node,
            None => panic!("parse error: no node to the left of binary operator"),
        }
        .kind
        {
            ASTNodeKind::Expression(exp) => exp,
            #[allow(unreachable_patterns)]
            _ => panic!("parse error: node to the left of binary operator isn't valid/supported"),
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

        let right: ASTExpression = match right {
            TokenKind::Integer(i) => ASTExpression {
                kind: ASTExpressionKind::IntegerLiteral(*i),
            },

            TokenKind::LParenthesis => todo!(),
            _ => panic!(),
        };

        Some((left, right))
    }

    fn parse_binary(&mut self) -> Option<()> {
        let (left, right) = self.extract_args_binary()?;

        // no need to verify if we're overriding the attribute since extract_args_binary() applies
        // .take() method on it
        self.node_buffer = Some(ASTNode::from(ASTExpression {
            kind: ASTExpressionKind::Binary(ASTBinaryExpression::new(
                match self
                    .peek(-1)
                    .expect("[OOPS]: I've been there before so...?")
                {
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
                    if let Some(_) = self.node_buffer {
                        panic!("expressions must be finished/separated by ';'");
                    }

                    self.node_buffer = Some(ASTNode::from(ASTExpression::from(*i)));
                }

                TokenKind::Plus | TokenKind::Hyphen | TokenKind::Asterisk | TokenKind::Slash => {
                    self.parse_binary()?;
                }

                TokenKind::SemiColon => {
                    if let Some(node) = self.node_buffer {
                        self.ast.push(node);
                        self.node_buffer = None;
                    }
                }

                _ => panic!("[TODO]: token no supported by the parser"),
            }

            self.counter += 1;
        }

        // If there are still a node in the buffer, means the last expression wasn't fully read
        // through or that it wasn't properly terminated (e.g. with a ';')
        if let None = self.node_buffer {
            Some(self.ast)
        } else {
            panic!("Parser buffer not empty yet... Last expr missing a ';'?");
        }
    }
}

impl From<Vec<TokenKind>> for Parser {
    fn from(tokens: Vec<TokenKind>) -> Self {
        Self {
            tokens,
            counter: 0,
            ast: AST::new(),
            node_buffer: None,
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
                node_buffer: None,
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
