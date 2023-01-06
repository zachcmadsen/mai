use std::fmt;

use logos::Logos;

#[derive(Clone, Copy, Debug, Logos)]
pub enum Token<'a> {
    #[error]
    #[regex(r"[ \n\t]+", logos::skip)]
    Error,
    #[regex("(0|[1-9][0-9]*)")]
    Integer(&'a str),
    #[token("+")]
    Plus,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Error => {
                unreachable!("Error tokens are handled during parsing")
            }
            Token::Integer(int) => write!(f, "{}", int),
            Token::Plus => write!(f, "+"),
        }
    }
}
