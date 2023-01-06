use std::fmt;

#[derive(Clone, Copy)]
pub enum Ty {
    Int,
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Ty::Int => "int",
        };

        write!(f, "{}", name)
    }
}

impl Ty {
    pub fn is_arithmetic(&self) -> bool {
        matches!(self, Ty::Int)
    }
}

pub struct Expr<'a> {
    pub ty: Ty,
    pub kind: ExprKind<'a>,
}

pub enum ExprKind<'a> {
    Add(&'a Expr<'a>, &'a Expr<'a>),
    Int(i32),
}
