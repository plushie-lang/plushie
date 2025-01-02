use std::{fs, path::PathBuf, process::Command};

use anyhow::Result;
use clap::{Parser, Subcommand};

use plushie::compile_to_js;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build { path: PathBuf },
    Run { path: PathBuf },
}

fn main() -> Result<()> {
    let Cli { command } = Cli::parse();

    match command {
        Commands::Build { mut path } => {
            let source = fs::read_to_string(&path)?;

            path.set_extension("js");

            fs::write(&path, compile_to_js(&source)?)?;
        }
        Commands::Run { path } => {
            let source = fs::read_to_string(&path)?;
            let source = compile_to_js(&source)?;

            Command::new("deno")
                .arg("eval")
                .arg(source)
                .spawn()?
                .wait()?;
        }
    }

    Ok(())
}
