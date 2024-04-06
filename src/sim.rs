use eyre::Result;

use crate::core::{Fire, Game, GameResult, Grid};

pub fn main() -> Result<()> {
    let mut game = Game::new(10);
    game.add_player("Alice");
    game.add_player("Bob");

    println!("Placing ships...");
    println!();
    for player in game.players.iter() {
        player.place_ships_randomly()?;
        println!(">>> {}", player.name);
        println!();
        print_grid(&player.grid);
        println!();
    }

    let game = game.start()?;

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
        Some(GameResult::Draw) => {
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

fn print_grid(grid: &Grid) {
    // Print header
    print!("   ");
    for x in 0..grid.size {
        print!("{:>2}", index_to_char(x));
    }
    println!();

    for (y, row) in grid.to_string().lines().enumerate() {
        print!("{:>2} ", y + 1);
        for ch in row.chars() {
            print!(" {}", ch);
        }
        println!();
    }
}

fn index_to_char(i: usize) -> char {
    (65u8 + i as u8) as char
}
