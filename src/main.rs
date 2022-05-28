use eyre::Result;
use slagskip::game::{Direction, Game, Ship};

fn main() -> Result<()> {
    let mut game = Game::new(vec!["Alice".to_string(), "Bob".to_string()], 10);

    for player in game.players.iter_mut() {
        println!("{}: Place your ships", player.name);

        let grid = &mut player.grid;
        grid.place_ship(Ship::Carrier, (1, 1), Direction::Horizontal)?;
        grid.place_ship(Ship::Battleship, (8, 2), Direction::Vertical)?;
        grid.place_ship(Ship::Cruiser, (3, 7), Direction::Vertical)?;
        grid.place_ship(Ship::Submarine, (0, 4), Direction::Horizontal)?;
        grid.place_ship(Ship::Destroyer, (5, 6), Direction::Horizontal)?;

        println!();
        grid.print();
        println!();
    }

    Ok(())
}
