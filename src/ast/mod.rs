pub mod parser;
pub mod tokenizer;

#[derive(Debug)]
pub(crate) enum ASTExpressionKind {
    IntegerLiteral(i64),
}

#[derive(Debug)]
pub(crate) struct ASTExpression {
    kind: ASTExpressionKind,
}

impl From<i64> for ASTExpression {
    fn from(value: i64) -> Self {
        Self {
            kind: ASTExpressionKind::IntegerLiteral(value),
        }
    }
}

#[derive(Debug)]
pub(crate) enum ASTNodeKind {
    Expression(ASTExpression),
}

impl From<ASTExpression> for ASTNodeKind {
    fn from(value: ASTExpression) -> Self {
        Self::Expression(value)
    }
}

#[derive(Debug)]
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
        Self { kind: value.into() }
    }
}

impl std::fmt::Display for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ASTNodeKind::Expression(expr) => {
                let _ = write!(f, "[ASTExpression] :: ");
                match expr.kind {
                    ASTExpressionKind::IntegerLiteral(i) => {
                        let _ = write!(f, "Integer literal :: value = {i}");
                    }
                }
            }
        };
        Ok(())
    }
}

#[derive(Debug)]
pub struct AST {
    nodes: Vec<ASTNode>,
}

impl AST {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn visualize(&self) {
        println!("{}", self);
    }
}

impl std::fmt::Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.nodes.iter().for_each(|node| {
            let _ = writeln!(f, "{node}");
        });

        Ok(())
    }
}
