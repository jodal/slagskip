use eyre::Result;
use slagskip::{cli::print_grid, game::NewGame};

fn main() -> Result<()> {
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

    while game.winner().is_none() {
        for turn in game.round() {
            for opponent in turn.opponents.iter() {
                match opponent.fire_at_random() {
                    Some((point, Some(ship))) => {
                        println!(
                            "{} fired at {} {} and hit {}!",
                            turn.player.name, opponent.name, point, ship
                        );
                    }
                    Some((point, None)) => {
                        println!(
                            "{} fired at {} {} and missed.",
                            turn.player.name, opponent.name, point
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

    if let Some(winner) = game.winner() {
        println!("{} won!", winner.name);
        println!();
    }

    for player in game.players.iter() {
        println!(">>> {}", player.name);
        println!();
        print_grid(&player.grid);
        println!();
    }

    Ok(())
}
