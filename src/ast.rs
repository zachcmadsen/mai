use std::fmt;

#[derive(Clone, Debug)]
pub struct FunctionDefinition {
    pub return_type: TypeSpecifier,
    pub name: String,
    pub body: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub enum TypeSpecifier {
    Int,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Compound(Vec<Statement>),
    Expr(ExprKind),
    Return(Option<ExprKind>),
}

#[derive(Clone, Debug)]
pub enum ExprKind {
    Binary(BinOpKind, Box<ExprKind>, Box<ExprKind>),
    Constant(Constant),
}

#[derive(Clone, Debug)]
pub enum BinOpKind {
    Add,
    Sub,
}

#[derive(Clone, Debug)]
pub enum Constant {
    Integer(String),
}

impl fmt::Display for BinOpKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            BinOpKind::Add => "+",
            BinOpKind::Sub => "-",
        };

        write!(f, "{}", op)
    }
}
