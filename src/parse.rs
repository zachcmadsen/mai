use anyhow::{anyhow, Result};
use bumpalo::Bump;
use lalrpop_util::{lalrpop_mod, ParseError};
use logos::Logos;

use crate::{ast::Expr, token::Token};

lalrpop_mod!(
    #[allow(clippy::all)]
    grammar
);

pub fn parse<'a>(source: &'a str, arena: &'a Bump) -> Result<&'a Expr<'a>> {
    let lexer =
        Token::lexer(source)
            .spanned()
            .map(|(token, span)| match token {
                Token::Error => Err(anyhow!(
                    "unexpected character(s) '{}'",
                    &source[span.start..span.end]
                )),
                _ => Ok((span.start, token, span.end)),
            });

    grammar::ExprParser::new()
        .parse(arena, lexer)
        .map_err(|err| match err {
            ParseError::UnrecognizedToken {
                token: (_, token, _),
                ..
            } => anyhow!("unexpected '{}'", token),
            ParseError::User { error } => error,
            _ => unimplemented!(),
        })
}
