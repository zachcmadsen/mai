use std::fmt;

use anyhow::{bail, Result};

use crate::ast;

// #[derive(Clone, Debug)]
pub struct FunctionDefinition {
    pub return_type: TypeSpecifier,
    pub name: String,
    pub body: Vec<Statement>,
}

// #[derive(Clone, Debug)]
pub enum TypeSpecifier {
    Int,
}

// #[derive(Clone, Debug)]
pub enum Statement {
    Compound(Vec<Statement>),
    Expr(Expr),
    Return(Option<Expr>),
}

pub struct Expr {
    pub ctype: CType,
    pub kind: ExprKind,
}

impl Expr {
    fn cast_to(self, ctype: CType) -> Expr {
        if self.ctype == ctype {
            self
        } else {
            Expr {
                ctype,
                kind: ExprKind::Cast(Box::new(self)),
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CType {
    Int,
    LongInt,
    UnsignedInt,
    UnsignedLongInt,
}

impl CType {
    pub fn is_arithmetic(&self) -> bool {
        matches!(
            self,
            CType::Int
                | CType::LongInt
                | CType::UnsignedInt
                | CType::UnsignedLongInt
        )
    }

    pub fn is_signed(&self) -> bool {
        matches!(self, CType::Int | CType::LongInt)
    }
}

impl fmt::Display for CType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            CType::Int => "int",
            CType::LongInt => "long int",
            CType::UnsignedInt => "unsigned int",
            CType::UnsignedLongInt => "unsigned long int",
        };

        write!(f, "{}", name)
    }
}

pub enum ExprKind {
    Binary(BinOpKind, Box<Expr>, Box<Expr>),
    Cast(Box<Expr>),
    Constant(Constant),
}

pub enum BinOpKind {
    Add,
    Sub,
}

#[derive(Clone, Debug)]
pub enum Constant {
    Int(i64),
    UnsignedInt(u64),
}

pub fn tc(ast: ast::FunctionDefinition) -> Result<FunctionDefinition> {
    let return_type = match ast.return_type {
        ast::TypeSpecifier::Int => TypeSpecifier::Int,
    };

    // TODO: Check for multiple returns in the same block.
    let mut statements = Vec::with_capacity(ast.body.len());
    for statement in ast.body {
        let statement = match statement {
            ast::Statement::Expr(expr) => Statement::Expr(tc_expr(expr)?),
            ast::Statement::Return(Some(expr)) => {
                let expr = tc_expr(expr)?;
                let return_type = match return_type {
                    TypeSpecifier::Int => CType::Int,
                };

                if expr.ctype == return_type {
                    Statement::Return(Some(expr))
                } else {
                    // We shouldn't unconditionally make a cast expression.
                    // Some casts are invalid?
                    Statement::Return(Some(expr.cast_to(return_type)))
                }
            }
            ast::Statement::Return(None) => Statement::Return(None),
            _ => todo!(),
        };

        statements.push(statement);
    }

    Ok(FunctionDefinition {
        return_type,
        name: ast.name,
        body: statements,
    })
}

fn tc_expr(kind: ast::ExprKind) -> Result<Expr> {
    match kind {
        ast::ExprKind::Binary(op, lhs, rhs) => tc_binary(op, *lhs, *rhs),
        ast::ExprKind::Constant(constant) => tc_constant(constant),
    }
}

fn tc_binary(
    op: ast::BinOpKind,
    lhs: ast::ExprKind,
    rhs: ast::ExprKind,
) -> Result<Expr> {
    let lhs = tc_expr(lhs)?;
    let rhs = tc_expr(rhs)?;

    if !lhs.ctype.is_arithmetic() || !rhs.ctype.is_arithmetic() {
        bail!(
            "invalid types for binary {} ('{}' and '{}')",
            op,
            lhs.ctype,
            rhs.ctype
        );
    }

    let op = match op {
        ast::BinOpKind::Add => BinOpKind::Add,
        ast::BinOpKind::Sub => BinOpKind::Sub,
    };
    // TODO: Perform integer promotions when shorts are added.
    let (lhs, rhs) = perform_arithmetic_conversions(lhs, rhs);

    Ok(Expr {
        ctype: lhs.ctype,
        kind: ExprKind::Binary(op, Box::new(lhs), Box::new(rhs)),
    })
}

fn tc_constant(constant: ast::Constant) -> Result<Expr> {
    match constant {
        ast::Constant::Integer(i) => {
            let (ctype, kind) = if let Ok(value) = i.parse::<i32>() {
                (CType::Int, ExprKind::Constant(Constant::Int(value as i64)))
            } else if let Ok(value) = i.parse::<i64>() {
                (CType::LongInt, ExprKind::Constant(Constant::Int(value)))
            } else if let Ok(value) = i.parse::<u64>() {
                (
                    CType::UnsignedLongInt,
                    ExprKind::Constant(Constant::UnsignedInt(value)),
                )
            } else {
                bail!("integer constant {} is too large", i);
            };

            Ok(Expr { ctype, kind })
        }
    }
}

fn perform_arithmetic_conversions(lhs: Expr, rhs: Expr) -> (Expr, Expr) {
    let ctype = match (lhs.ctype, rhs.ctype) {
        (CType::UnsignedLongInt, _)
        | (_, CType::UnsignedLongInt)
        | (CType::LongInt, CType::UnsignedInt)
        | (CType::UnsignedInt, CType::LongInt) => CType::UnsignedLongInt,
        (CType::LongInt, _) | (_, CType::LongInt) => CType::LongInt,
        (CType::UnsignedInt, _) | (_, CType::UnsignedInt) => {
            CType::UnsignedInt
        }
        (CType::Int, CType::Int) => CType::Int,
    };

    (lhs.cast_to(ctype), rhs.cast_to(ctype))
}
