use anyhow::Result;
use clap::Parser;
use std::{fs, path::PathBuf};

use penny::{compile_to_js, error::Error, parser::ProgramParser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    source: PathBuf,
}

fn main() -> Result<()> {
    let Args { mut source } = Args::parse();
    let src = fs::read_to_string(&source)?;

    let js = compile_to_js(&src);

    source.set_extension("js");

    fs::write(&source, js)?;

    Ok(())
}
