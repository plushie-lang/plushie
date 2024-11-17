use anyhow::Result;
use clap::Parser;
use lalrpop_util::lalrpop_mod;
use std::{fs, path::PathBuf};

mod ast;

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

    let program = parser.parse(&source).expect("Failed to parse code");

    dbg!(program);

    Ok(())
}
