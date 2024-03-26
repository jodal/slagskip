use eyre::Result;
use slagskip::{
    cli::print_grid,
    game::{Direction, Game, Ship},
};

fn main() -> Result<()> {
    let game = Game::new(vec!["Alice".to_string(), "Bob".to_string()], 10);

    for player in game.players.iter() {
        println!("{}: Place your ships", player.name);

        player
            .grid
            .place_ship(Ship::Carrier, (1, 1), Direction::Horizontal)?;
        player
            .grid
            .place_ship(Ship::Battleship, (8, 2), Direction::Vertical)?;
        player
            .grid
            .place_ship(Ship::Cruiser, (3, 7), Direction::Vertical)?;
        player
            .grid
            .place_ship(Ship::Submarine, (0, 4), Direction::Horizontal)?;
        player
            .grid
            .place_ship(Ship::Destroyer, (5, 6), Direction::Horizontal)?;

        println!();
        print_grid(&player.grid);
        println!();
    }

    for turn in game.round() {
        println!("{}: Fire!", turn.player.name);

        for opponent in turn.opponents.iter() {
            let (x, y) = opponent.grid.random_square();
            println!(
                "{} fires at {} ({}, {})",
                turn.player.name, opponent.name, x, y
            );

            match opponent.grid.fire_at(x, y) {
                Some(ship) => println!("{} hit {}!", turn.player.name, ship),
                None => println!("{} missed!", turn.player.name),
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
