use slagskip::game::{Board, Ship};
use slagskip::tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut board = Board::new(10);
    board.place_ship(Ship::Carrier, (0, 0), true)?;
    board.place_ship(Ship::Battleship, (2, 1), true)?;
    board.place_ship(Ship::Submarine, (0, 2), true)?;

    // dbg!(board);
    board.print();

    tui::run()?;

    Ok(())
}
