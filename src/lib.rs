pub mod ast;
pub mod backend;
pub mod lexer;
pub mod parser;
pub mod tc;

use std::io::Write;
use std::process::Command;

use anyhow::{Context, Result};

use crate::backend::Backend;
use crate::lexer::Lexer;

pub fn run(source: &str) -> Result<()> {
    let lexer = Lexer::new(source);
    let ast = parser::parse(lexer)?;
    let tast = tc::tc(ast)?;
    let bytes = Backend::new(tast)
        .context("failed to initialize backend")?
        .compile()
        .context("failed to compile")?;

    let mut file = tempfile::NamedTempFile::new()
        .context("failed to create a temporary object file")?;
    file.write_all(&bytes)
        .context("failed to write to an object file")?;

    // TODO: Handle unsuccessful statuses.
    Command::new("gcc").args([file.as_ref()]).status()?;

    Ok(())
}
