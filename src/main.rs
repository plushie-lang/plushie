use anyhow::Result;
use clap::Parser;
use pest::Parser as _;
use std::{fs, path::PathBuf};

mod parser;
// mod jit;

use parser::{PennyParser, Rule};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    source: PathBuf,
}

fn main() -> Result<()> {
    let Args { source } = Args::parse();
    let source = fs::read_to_string(source)?;

    let pairs = PennyParser::parse(Rule::program, &source)?;
    println!("parse tree = {:#?}", pairs);

    for pair in pairs {
        match pair.as_rule() {
            rule => {
                dbg!(rule);
            }
        }
    }

    Ok(())
}
