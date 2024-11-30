use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};

use penny::compile_to_js;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build { path: PathBuf },
    Run,
}

fn main() -> Result<()> {
    let Cli { command } = Cli::parse();

    match command {
        Commands::Build { mut path } => {
            let source = fs::read_to_string(&path)?;

            path.set_extension("js");

            fs::write(&path, compile_to_js(&source))?;
        }
        Commands::Run => todo!(),
    }

    Ok(())
}
