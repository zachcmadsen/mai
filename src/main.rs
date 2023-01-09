use std::{fs, path::Path};

use clap::{ColorChoice, Parser};

#[derive(Parser)]
#[command(version, color = ColorChoice::Never)]
struct Args {
    filename: String,
}

fn main() {
    let args = Args::parse();

    // TODO: See if the lexer can accept a reader instead of a string.
    let Ok(source) = fs::read_to_string(&args.filename) else {
        eprintln!("error: failed to read from {}", args.filename);
        return;
    };

    let Some(file_stem) = Path::new(&args.filename).file_stem() else {
        eprintln!("error: failed to extract the stem from {}", args.filename);
        return;
    };

    let Some(out) = file_stem.to_str() else {
        eprintln!(
            "error: file stem of '{}' contains invalid Unicode",
            args.filename);
        return;
    };

    if let Err(err) = mai::run(&source, out) {
        eprintln!("{}: error: {}", &args.filename, err);
    }
}
