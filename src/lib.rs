mod ast;
mod codegen;
mod parse;
mod tast;
mod tc;
mod token;

use std::{io::Write, process::ExitStatus};

use anyhow::{Context, Result};
use bumpalo::Bump;
use tempfile::NamedTempFile;

pub fn run(source: &str, out: &str) -> Result<()> {
    let ast_arena = Bump::new();
    let ast = parse::parse(source, &ast_arena)?;

    let tast_arena = Bump::new();
    let tast = tc::tc(ast, &tast_arena)?;
    drop(ast_arena);

    let bytes =
        codegen::gen(tast).context("failed to generate object code")?;

    let mut file =
        NamedTempFile::new().context("failed to create a temporary file")?;
    file.write_all(&bytes)
        .context("failed to write to a file")?;

    // TODO: Handle non-success statuses.
    link(file, out)?;

    Ok(())
}

#[cfg(not(windows))]
fn link(file: NamedTempFile, out: &str) -> Result<ExitStatus> {
    use std::process::Command;

    // TODO: Set the output filename.
    Ok(Command::new("cc").arg(file.as_ref()).status()?)
}

#[cfg(windows)]
fn link(file: NamedTempFile, out: &str) -> Result<ExitStatus> {
    use anyhow::bail;

    let Some(mut linker) = cc::windows_registry::find(
        &codegen::HOST_TRIPLE.to_string(),
        "link.exe",
    ) else {
        bail!("link.exe could not be found");
    };

    // Close the file so that the linker can open it.
    let path = file.into_temp_path();

    Ok(linker
        .arg(&path)
        .arg(format!("/out:{}.exe", out))
        .arg("/entry:main")
        .arg("/nologo")
        .status()?)
}
