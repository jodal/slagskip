use eyre::{eyre, Result};
use std::fmt;

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub grid_size: usize,
}

impl Game {
    pub fn new(players: Vec<String>, grid_size: usize) -> Self {
        Self {
            players: players
                .into_iter()
                .map(|name| Player::new(name, grid_size))
                .collect(),
            grid_size,
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub grid: Grid,
}

impl Player {
    fn new(name: String, grid_size: usize) -> Self {
        Self {
            name,
            grid: Grid::new(grid_size),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    fn step(&self) -> (usize, usize) {
        match self {
            Self::Horizontal => (1, 0),
            Self::Vertical => (0, 1),
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
        match self {
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

    fn at(&mut self, x: usize, y: usize) -> Option<&mut Square> {
        if (x >= self.size) || (y >= self.size) {
            return None;
        }
        Some(&mut self.squares[x][y])
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
            let pos_x = x + i * step_x;
            let pos_y = y + i * step_y;

            match self.at(pos_x, pos_y) {
                None => return Err(eyre!("{} is out of bounds", ship)),
                Some(square) => {
                    match square.ship {
                        Some(existing_ship) => {
                            return Err(eyre!("{} overlaps with {}", ship, existing_ship))
                        }
                        None => {} // All good: Square is within bounds and empty
                    }
                }
            }
        }

        // Actually place the ship
        for i in 0..ship.length() {
            let pos_x = x + i * step_x;
            let pos_y = y + i * step_y;
            self.at(pos_x, pos_y).unwrap().place_ship(ship);
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
