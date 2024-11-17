use anyhow::Result;
use clap::Parser;
use lalrpop_util::lalrpop_mod;
use std::{fs, path::PathBuf};

mod ast;
mod error;

use error::Error;

lalrpop_mod!(pub penny);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    source: PathBuf,
}

fn main() -> Result<()> {
    let Args { source } = Args::parse();
    let source = fs::read_to_string(source)?;

    let parser = penny::ProgramParser::new();

    let program = parser
        .parse(&source)
        .map_err(|error| Error::from_parse_error(&source, error))?;

    dbg!(program);

    Ok(())
}
