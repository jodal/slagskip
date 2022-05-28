use eyre::Result;
use slagskip::game::{Direction, Grid, Ship};

fn main() -> Result<()> {
    let mut grid = Grid::new(10);

    grid.place_ship(Ship::Carrier, (1, 1), Direction::Horizontal)?;
    grid.place_ship(Ship::Battleship, (8, 2), Direction::Vertical)?;
    grid.place_ship(Ship::Cruiser, (3, 7), Direction::Vertical)?;
    grid.place_ship(Ship::Submarine, (0, 4), Direction::Horizontal)?;
    grid.place_ship(Ship::Destroyer, (5, 6), Direction::Horizontal)?;

    grid.print();

    Ok(())
}
