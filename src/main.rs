use slagskip::game::{Board, Ship};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut board = Board::new(10);
    board.place_ship(Ship::Carrier, (1, 1), true)?;
    board.place_ship(Ship::Battleship, (8, 2), false)?;
    board.place_ship(Ship::Cruiser, (3, 7), false)?;
    board.place_ship(Ship::Submarine, (0, 4), true)?;
    board.place_ship(Ship::Destroyer, (5, 6), true)?;

    // dbg!(&board);
    board.print();

    Ok(())
}
