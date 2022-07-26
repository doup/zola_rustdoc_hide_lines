use std::path::Path;

use clap::{ArgEnum, Parser};
use zola_rustdoc_hide_lines::{formatter, linter};

#[derive(Parser)]
#[clap(author, about, long_about = None)]
pub struct Args {
    #[clap(arg_enum)]
    mode: Mode,
    path: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Mode {
    Lint,
    Format,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);

    match args.mode {
        Mode::Lint => linter::run(path),
        Mode::Format => formatter::run(path),
    }
}
