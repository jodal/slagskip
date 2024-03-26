use eyre::Result;
use slagskip::{
    cli::print_grid,
    game::{Direction, Game, Ship},
};

fn main() -> Result<()> {
    let mut game = Game::new(vec!["Alice".to_string(), "Bob".to_string()], 10);

    for (i, player_name) in game.players.iter().enumerate() {
        println!("{}: Place your ships", player_name);

        let grid = &mut game.grids[i];
        grid.place_ship(Ship::Carrier, (1, 1), Direction::Horizontal)?;
        grid.place_ship(Ship::Battleship, (8, 2), Direction::Vertical)?;
        grid.place_ship(Ship::Cruiser, (3, 7), Direction::Vertical)?;
        grid.place_ship(Ship::Submarine, (0, 4), Direction::Horizontal)?;
        grid.place_ship(Ship::Destroyer, (5, 6), Direction::Horizontal)?;

        println!();
        print_grid(grid);
        println!();
    }

    for (i, player_name) in game.players.iter().enumerate() {
        println!("{}: Fire!", player_name);

        for (j, opponent_grid) in game.grids.iter_mut().enumerate() {
            if i == j {
                continue; // Don't fight yourself
            }

            let opponent_name = &game.players[j];
            let (x, y) = opponent_grid.random_square();
            println!("{} fires at {} ({}, {})", player_name, opponent_name, x, y);

            match opponent_grid.fire_at(x, y) {
                Some(ship) => println!("{} hit {}!", player_name, ship),
                None => println!("{} missed!", player_name),
            }
        }
    }

    for (i, player_name) in game.players.iter().enumerate() {
        let grid = &mut game.grids[i];

        println!("Grid of {}", player_name);
        println!();
        print_grid(grid);
        println!();
    }

    Ok(())
}
