mod ast;
mod codegen;
mod parse;
mod tast;
mod tc;
mod token;

use std::io::Write;
use std::process::Command;

use anyhow::{Context, Result};

pub fn run(source: &str) -> Result<()> {
    let ast_arena = bumpalo::Bump::new();
    let ast = parse::parse(source, &ast_arena)?;

    let tast_arena = bumpalo::Bump::new();
    let tast = tc::tc(ast, &tast_arena)?;
    drop(ast_arena);

    let bytes =
        codegen::gen(tast).context("failed to generate object code")?;

    let mut file = tempfile::NamedTempFile::new()
        .context("failed to create a temporary file")?;
    file.write_all(&bytes)
        .context("failed to write to a file")?;

    Command::new("cc").args([file.as_ref()]).spawn()?;

    Ok(())
}
