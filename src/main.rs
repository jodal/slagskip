use eyre::Result;
use slagskip::{
    cli::print_grid,
    game::{Fire, GameResult, NewGame},
};

use clap::{Parser, Subcommand};

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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Sim => sim(),
    }
}

fn sim() -> Result<()> {
    let mut new_game = NewGame::new(10);
    new_game.add_player("Alice");
    new_game.add_player("Bob");

    println!("Placing ships...");
    println!();
    for player in new_game.players.iter() {
        player.place_ships_randomly()?;
        println!(">>> {}", player.name);
        println!();
        print_grid(&player.grid);
        println!();
    }

    let game = new_game.start()?;

    while game.result().is_none() {
        for turn in game.round() {
            for opponent in turn.opponents.iter() {
                match opponent.fire_at_random() {
                    Some((point, Fire::Miss)) => {
                        println!(
                            "{} fired at {} {} and missed.",
                            turn.player.name, opponent.name, point
                        );
                    }
                    Some((point, Fire::Hit)) => {
                        println!(
                            "{} fired at {} {} and hit!",
                            turn.player.name, opponent.name, point
                        );
                    }
                    Some((point, Fire::Sunk(ship))) => {
                        println!(
                            "{} fired at {} {} and sunk a {}!",
                            turn.player.name, opponent.name, point, ship
                        );
                    }
                    None => {
                        println!("No more cells to hit!")
                    }
                }
            }
        }
        println!();
    }

    match game.result() {
        Some(GameResult::Winner(winner)) => {
            println!("{} won!", winner.name);
            println!();
        }
        Some(GameResult::Tie) => {
            println!("Game ended in a tie!");
            println!();
        }
        None => {}
    }

    for player in game.players.iter() {
        println!(">>> {}", player.name);
        println!();
        print_grid(&player.grid);
        println!();
    }

    Ok(())
}
