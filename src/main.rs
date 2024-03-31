use eyre::Result;

use clap::{Parser, Subcommand};
use slagskip::sim;
use slagskip::tui;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Simulate a full game to exercise game engine.
    Sim,

    /// Play game in terminal UI.
    Tui,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Sim => sim::main(),
        Commands::Tui => tui::main(),
    }
}
