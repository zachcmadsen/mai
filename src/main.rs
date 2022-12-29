use clap::{ColorChoice, Parser};

#[derive(Parser)]
#[command(version, color = ColorChoice::Never)]
struct Args {
    filename: String,
}

fn main() {
    let args = Args::parse();

    let source = std::fs::read_to_string(args.filename).unwrap();
    mai::run(&source);
}
