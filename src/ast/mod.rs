use core::panic;
use std::i64;

use self::tokenizer::TokenKind;

pub mod parser;
pub mod tokenizer;

pub(crate) enum ASTExpressionKind {
    IntegerLiteral(i64),
    Binary(ASTBinaryExpression),
}

pub(crate) enum BinaryOperatorKind {
    Plus,
    Minus,
    Mult,
    Div,
}

impl std::fmt::Display for BinaryOperatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperatorKind::Plus => write!(f, "+"),
            BinaryOperatorKind::Minus => write!(f, "-"),
            BinaryOperatorKind::Mult => write!(f, "*"),
            BinaryOperatorKind::Div => write!(f, "/"),
        }
    }
}

pub(crate) struct ASTBinaryExpression {
    operator: BinaryOperatorKind,
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
}

impl ASTBinaryExpression {
    fn new(operator: BinaryOperatorKind, left: ASTExpression, right: ASTExpression) -> Self {
        Self {
            operator,
            left: left.into(),
            right: right.into(),
        }
    }
}

pub(crate) struct ASTExpression {
    kind: ASTExpressionKind,
}

impl TryFrom<&tokenizer::TokenKind> for ASTExpression {
    type Error = ();

    fn try_from(token: &tokenizer::TokenKind) -> Result<Self, Self::Error> {
        match token {
            TokenKind::Integer(i) => Ok(Self {
                kind: ASTExpressionKind::IntegerLiteral(*i),
            }),
            _ => Err(()),
        }
    }
}

impl From<i64> for ASTExpression {
    fn from(i: i64) -> Self {
        Self {
            kind: ASTExpressionKind::IntegerLiteral(i),
        }
    }
}

impl TryFrom<ASTNode> for ASTExpression {
    type Error = ();

    fn try_from(value: ASTNode) -> Result<Self, Self::Error> {
        match value.kind {
            ASTNodeKind::Expression(exp) => Ok(exp),
            _ => Err(()),
        }
    }
}

pub(crate) enum ASTNodeKind {
    Expression(ASTExpression),
}

impl From<ASTExpression> for ASTNodeKind {
    fn from(value: ASTExpression) -> Self {
        Self::Expression(value)
    }
}

pub struct ASTNode {
    kind: ASTNodeKind,
}

impl From<ASTNodeKind> for ASTNode {
    fn from(value: ASTNodeKind) -> Self {
        Self { kind: value }
    }
}

impl From<ASTExpression> for ASTNode {
    fn from(value: ASTExpression) -> Self {
        Self {
            kind: ASTNodeKind::Expression(value),
        }
    }
}

pub struct AST {
    nodes: Vec<ASTNode>,
}

impl AST {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn push(&mut self, value: ASTNode) {
        self.nodes.push(value)
    }

    fn pop_last(&mut self) -> Option<ASTNode> {
        self.nodes.pop()
    }

    pub fn visualize(&self) {
        let mut visualizer = ASTVisualizer::new();

        self.nodes.iter().for_each(|node| {
            visualizer.visit_node(&node);
        })
    }
}

trait ASTVisitor {
    fn visit_node(&mut self, node: &ASTNode) {
        match &node.kind {
            ASTNodeKind::Expression(e) => self.visit_expression(&e),

            // NOTE: remove on final build
            #[allow(unreachable_patterns)]
            _ => {
                panic!("[ast::ASTVisitor]: visit_node() not implemented for given ast::ASTNodeKind")
            }
        }
    }

    fn visit_expression(&mut self, exp: &ASTExpression);
}

struct ASTVisualizer {
    indent: usize,
}

impl ASTVisualizer {
    fn new() -> Self {
        Self { indent: 0 }
    }
}

impl ASTVisitor for ASTVisualizer {
    fn visit_expression(&mut self, exp: &ASTExpression) {
        match &exp.kind {
            ASTExpressionKind::IntegerLiteral(i) => {
                println!("{}[Integer Literal]: {}", " ".repeat(self.indent), i);
            }

            ASTExpressionKind::Binary(exp) => {
                println!(
                    "{}Binary Operator: {}",
                    " ".repeat(self.indent),
                    exp.operator
                );
                self.indent += 4;
                println!("{}Left operand: ", " ".repeat(self.indent));
                self.indent += 4;
                self.visit_expression(&exp.left);
                println!("{}Right operand: ", " ".repeat(self.indent - 4));
                self.visit_expression(&exp.right);

                self.indent -= 8;
            }
        }
    }
}
