use eyre::{eyre, Result};
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    fn step(&self) -> (usize, usize) {
        match self {
            Direction::Horizontal => (1, 0),
            Direction::Vertical => (0, 1),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Ship {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
}

impl Ship {
    pub fn length(&self) -> usize {
        match &self {
            Self::Carrier => 5,
            Self::Battleship => 4,
            Self::Cruiser => 3,
            Self::Submarine => 3,
            Self::Destroyer => 2,
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
pub struct Grid {
    size: usize,
    squares: Vec<Vec<Square>>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        Grid {
            size: size,
            squares: vec![vec![Square::new(); size]; size],
        }
    }

    pub fn place_ship(
        &mut self,
        ship: Ship,
        (x, y): (usize, usize),
        direction: Direction,
    ) -> Result<()> {
        let (step_x, step_y) = direction.step();

        // Validate the placement
        for i in 0..ship.length() {
            let square_x = x + i * step_x;
            let square_y = y + i * step_y;

            if (square_x >= self.size) || (square_y >= self.size) {
                return Err(eyre!("{} is out of bounds", ship));
            }

            let square = &mut self.squares[square_x][square_y];

            if let Some(existing_ship) = square.ship {
                return Err(eyre!("{} overlaps with {}", ship, existing_ship));
            }
        }

        // Actually place the ship
        for i in 0..ship.length() {
            let square = &mut self.squares[x + i * step_x][y + i * step_y];
            square.place_ship(ship);
        }

        Ok(())
    }

    pub fn print(&self) {
        // Print header
        print!("   ");
        for x in 0..self.size {
            print!("{:>2}", index_to_char(x));
        }
        println!("");

        for y in 0..self.size {
            print!("{:>2} ", y + 1);
            for x in 0..self.size {
                match self.squares[x][y].ship {
                    Some(_ship) => print!(" O"),
                    None => print!(" ."),
                }
            }
            println!("");
        }
    }
}

fn index_to_char(i: usize) -> char {
    (65u8 + i as u8) as char
}
