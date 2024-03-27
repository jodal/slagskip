use eyre::Result;
use slagskip::{cli::print_grid, game::NewGame};

fn main() -> Result<()> {
    let mut new_game = NewGame::new(10);
    new_game.add_player("Alice");
    new_game.add_player("Bob");

    for player in new_game.players.iter() {
        println!("{}: Place your ships", player.name);
        player.place_ships_randomly()?;
        println!();
        print_grid(&player.grid);
        println!();
    }

    let game = new_game.start()?;

    for turn in game.round() {
        println!("{}: Fire!", turn.player.name);

        for opponent in turn.opponents.iter() {
            match opponent.fire_at_random() {
                Some(((x, y), Some(ship))) => {
                    println!(
                        "{} fired at {},{} and hit {}!",
                        turn.player.name, x, y, ship
                    );
                }
                Some(((x, y), None)) => {
                    println!("{} fired at {},{} and hit nothing.", turn.player.name, x, y);
                }
                None => {
                    println!("No more cells to hit!")
                }
            }
        }
    }

    for player in game.players.iter() {
        println!("Grid of {}", player.name);
        println!();
        print_grid(&player.grid);
        println!();
    }

    Ok(())
}
