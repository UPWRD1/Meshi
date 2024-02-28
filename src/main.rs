use anyhow::Result;
use clap::{Parser, Subcommand};

pub mod core;

use core::setup;

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: Scmd,
}

#[derive(Debug, Subcommand)]
enum Scmd {
    Setup,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Scmd::Setup => {
            setup::setup();
        }
    }
    Ok(())
}
