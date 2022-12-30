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
    Identifier(String),
}

#[derive(Clone, Debug)]
pub enum BinOpKind {
    Add,
    Sub,
}

#[derive(Clone, Debug)]
pub enum Constant {
    Integer(i64),
}
