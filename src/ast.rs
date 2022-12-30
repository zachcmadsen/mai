#[derive(Clone, Debug)]
pub struct FunctionDefinition {
    pub return_type: TypeSpecifier,
    pub name: String,
    pub body: CompoundStatement,
}

#[derive(Clone, Debug)]
pub enum TypeSpecifier {
    Int,
}

#[derive(Clone, Debug)]
pub struct CompoundStatement(pub Vec<Statement>);

#[derive(Clone, Debug)]
pub enum Statement {
    Compound(CompoundStatement),
    Expr(Expr),
    Return(Option<Expr>),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Assignment(TypeSpecifier, String, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Constant(Constant),
    Identifier(String),
}

#[derive(Clone, Debug)]
pub enum Constant {
    Integer(i64),
}
