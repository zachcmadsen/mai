pub mod ast;
pub mod backend;
pub mod lexer;

use std::io::Write;
use std::process::Command;

use lalrpop_util::lalrpop_mod;

use crate::backend::Backend;
use crate::lexer::Lexer;

lalrpop_mod!(pub parser);

pub fn run(source: &str) {
    let lexer = Lexer::new(&source);
    let function_definition = parser::FunctionDefinitionParser::new()
        .parse(lexer)
        .unwrap();
    let bytes = Backend::new(function_definition).compile();

    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(&bytes).unwrap();

    // TODO: Handle unsuccessful statuses.
    Command::new("gcc").args([file.as_ref()]).status().unwrap();
}
