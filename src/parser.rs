use anyhow::{anyhow, Result};
use lalrpop_util::{lalrpop_mod, ParseError};

use crate::{
    ast::FunctionDefinition,
    lexer::{Lexer, Token},
};

lalrpop_mod!(
    #[allow(clippy::all)]
    parser
);

pub fn parse(lexer: Lexer) -> Result<FunctionDefinition> {
    parser::FunctionDefinitionParser::new()
        .parse(lexer)
        .map_err(|err| match err {
            ParseError::UnrecognizedToken {
                token: (_, token, _),
                ..
            } => {
                if matches!(token, Token::Integer(_) | Token::Identifier(_)) {
                    anyhow!("unexpected {}", token)
                } else {
                    anyhow!("unexpected '{}'", token)
                }
            }
            ParseError::User { error } => error,
            _ => unimplemented!(),
        })
}
