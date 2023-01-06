use anyhow::{anyhow, bail, Result};
use bumpalo::Bump;

use crate::{
    ast,
    tast::{Expr, ExprKind, Ty},
};

pub fn tc<'ast, 'tast>(
    expr: &'ast ast::Expr,
    arena: &'tast Bump,
) -> Result<&'tast Expr<'tast>> {
    match expr {
        ast::Expr::Add(lhs, rhs) => {
            let lhs = tc(lhs, arena)?;
            let rhs = tc(rhs, arena)?;

            if !lhs.ty.is_arithmetic() || !rhs.ty.is_arithmetic() {
                bail!("invalid types for + ('{}' and '{}')", lhs.ty, rhs.ty);
            }

            Ok(arena.alloc(Expr {
                ty: lhs.ty,
                kind: ExprKind::Add(lhs, rhs),
            }))
        }
        ast::Expr::Integer(int) => {
            if let Ok(int) = int.parse::<i32>() {
                Ok(arena.alloc(Expr {
                    ty: Ty::Int,
                    kind: ExprKind::Int(int),
                }))
            } else {
                Err(anyhow!("integer constant {} is too large", int))
            }
        }
    }
}
