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

    for player in game.players.iter() {
        println!("{}: Fire!", player.name);

        for opponent in game.players.iter() {
            if opponent == player {
                continue; // Don't fight yourself
            }

            let (x, y) = opponent.grid.random_square();
            println!("{} fires at {} ({}, {})", player.name, opponent.name, x, y);

            match opponent.grid.fire_at(x, y) {
                Some(ship) => println!("{} hit {}!", player.name, ship),
                None => println!("{} missed!", player.name),
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
