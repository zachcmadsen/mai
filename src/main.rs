use std::fs;

use clap::{ColorChoice, Parser};

#[derive(Parser)]
#[command(version, color = ColorChoice::Never)]
struct Args {
    filename: String,
}

fn main() {
    let args = Args::parse();

    // TODO: See if the lexer can accept a reader instead of a string.
    let source = match fs::read_to_string(&args.filename) {
        Ok(source) => source,
        Err(_) => {
            eprintln!("error: failed to read from {}", args.filename);
            return;
        }
    };

    if let Err(err) = mai::run(&source) {
        eprintln!("{}: error: {}", &args.filename, err);
    }
}
