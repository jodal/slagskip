use std::fmt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut board = Board::new(10);
    board.place_ship(Ship::Carrier, (0, 0), true)?;
    board.place_ship(Ship::Battleship, (2, 1), true)?;
    board.place_ship(Ship::Submarine, (0, 2), true)?;

    // dbg!(board);
    board.print();

    Ok(())
}

#[derive(Copy, Clone, Debug)]
pub enum Ship {
    Carrier,
    Battleship,
    Cruiser,
    Destroyer,
    Submarine,
}

impl Ship {
    pub fn size(&self) -> usize {
        match &self {
            Self::Carrier => 5,
            Self::Battleship => 4,
            Self::Cruiser => 3,
            Self::Destroyer => 2,
            Self::Submarine => 1,
        }
    }
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
struct Square {
    ship: Option<Ship>,
    hit: bool,
}

impl Square {
    fn new() -> Self {
        Square {
            ship: None,
            hit: false,
        }
    }

    fn place_ship(&mut self, ship: Ship) {
        self.ship = Some(ship);
    }
}

#[derive(Debug)]
pub struct Board {
    size: usize,
    squares: Vec<Vec<Square>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Board {
            size: size,
            squares: vec![vec![Square::new(); size]; size],
        }
    }

    pub fn place_ship(
        &mut self,
        ship: Ship,
        (x, y): (usize, usize),
        horizontal: bool,
    ) -> Result<(), String> {
        let size = ship.size();
        let ship_squares: Vec<&Square> = (0..size)
            .map(|i| {
                if horizontal {
                    &self.squares[x + i][y]
                } else {
                    &self.squares[x][y + i]
                }
            })
            .collect();

        if !ship_squares.iter().all(|&square| square.ship.is_none()) {
            return Err(format!("Ship {} overlaps", ship).into());
        }

        for i in 0..size {
            let square = if horizontal {
                &mut self.squares[x + i][y]
            } else {
                &mut self.squares[x][y + i]
            };
            square.place_ship(ship);
        }

        Ok(())
    }

    pub fn print(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                match self.squares[x][y].ship {
                    Some(_ship) => print!("S"),
                    None => print!("."),
                }
            }
            println!("");
        }
    }
}
