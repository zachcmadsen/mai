use std::fmt;

use anyhow::{anyhow, Result};
use logos::Logos;

#[derive(Clone, Debug, Logos)]
pub enum Token {
    // Keywords
    #[token("auto")]
    Auto,
    #[token("break")]
    Break,
    #[token("case")]
    Case,
    #[token("char")]
    Char,
    #[token("const")]
    Const,
    #[token("continue")]
    Continue,
    #[token("default")]
    Default,
    #[token("do")]
    Do,
    #[token("double")]
    Double,
    #[token("else")]
    Else,
    #[token("enum")]
    Enum,
    #[token("extern")]
    Extern,
    #[token("float")]
    Float,
    #[token("for")]
    For,
    #[token("goto")]
    Goto,
    #[token("if")]
    If,
    #[token("int")]
    Int,
    #[token("long")]
    Long,
    #[token("register")]
    Register,
    #[token("return")]
    Return,
    #[token("short")]
    Short,
    #[token("signed")]
    Signed,
    #[token("sizeof")]
    Sizeof,
    #[token("static")]
    Static,
    #[token("struct")]
    Struct,
    #[token("switch")]
    Switch,
    #[token("typedef")]
    Typedef,
    #[token("union")]
    Union,
    #[token("unsigned")]
    Unsigned,
    #[token("void")]
    Void,
    #[token("volatile")]
    Volatile,
    #[token("while")]
    While,

    // Operators
    #[token("+")]
    Plus,

    // Punctuators
    #[token(",")]
    Comma,
    #[token("{")]
    LeftBrace,
    #[token("(")]
    LeftParen,
    #[token("}")]
    RightBrace,
    #[token(")")]
    RightParen,
    #[token(";")]
    Semi,

    // Constants
    #[regex("(0|[1-9][0-9]*)", |lex| lex.slice().parse::<i64>().unwrap())]
    Integer(i64),

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[error]
    #[regex(r"[ \n\t]+", logos::skip)]
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_name = match self {
            Token::Auto => "auto",
            Token::Break => "break",
            Token::Case => "case",
            Token::Char => "char",
            Token::Const => "const",
            Token::Continue => "continue",
            Token::Default => "default",
            Token::Do => "do",
            Token::Double => "double",
            Token::Else => "else",
            Token::Enum => "enum",
            Token::Extern => "extern",
            Token::Float => "float",
            Token::For => "for",
            Token::Goto => "goto",
            Token::If => "if",
            Token::Int => "int",
            Token::Long => "long",
            Token::Register => "register",
            Token::Return => "return",
            Token::Short => "short",
            Token::Signed => "signed",
            Token::Sizeof => "sizeof",
            Token::Static => "static",
            Token::Struct => "struct",
            Token::Switch => "switch",
            Token::Typedef => "typedef",
            Token::Union => "union",
            Token::Unsigned => "unsigned",
            Token::Void => "void",
            Token::Volatile => "volatile",
            Token::While => "while",

            Token::Plus => "+",

            Token::Comma => ",",
            Token::Semi => ";",
            Token::LeftBrace => "{",
            Token::LeftParen => "(",
            Token::RightBrace => "}",
            Token::RightParen => ")",

            Token::Integer(_) => "integer literal",

            Token::Identifier(_) => "identifier",

            Token::Error => {
                unreachable!("Error tokens shouldn't be displayed")
            }
        };

        write!(f, "{}", token_name)
    }
}

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            inner: Token::lexer(source),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<(usize, Token, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(Token::Error) => Some(Err(anyhow!(
                "unexpected character(s) '{}'",
                self.inner.slice()
            ))),
            Some(token) => {
                let span = self.inner.span();
                Some(Ok((span.start, token, span.end)))
            }
            None => None,
        }
    }
}
