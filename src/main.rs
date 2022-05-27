use eyre::Result;
use slagskip::game::{Board, Direction, Ship};

fn main() -> Result<()> {
    let mut board = Board::new(10);
    board.place_ship(Ship::Carrier, (1, 1), Direction::Horizontal)?;
    board.place_ship(Ship::Battleship, (8, 2), Direction::Vertical)?;
    board.place_ship(Ship::Cruiser, (3, 7), Direction::Vertical)?;
    board.place_ship(Ship::Submarine, (0, 4), Direction::Horizontal)?;
    board.place_ship(Ship::Destroyer, (5, 6), Direction::Horizontal)?;

    // dbg!(&board);
    board.print();

    Ok(())
}
