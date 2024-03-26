use eyre::{eyre, Result};
use rand::{thread_rng, Rng};
use std::{cell::RefCell, fmt};

#[derive(Debug)]
pub struct Game {
    pub players: Vec<String>,
    pub grids: Vec<Grid>,
}

impl Game {
    pub fn new(players: Vec<String>, grid_size: usize) -> Self {
        let num_players = players.len();
        Self {
            players,
            grids: (0..num_players).map(|_| Grid::new(grid_size)).collect(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    pub size: usize,
    pub squares: RefCell<Vec<Vec<Square>>>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        Grid {
            size,
            squares: RefCell::new(vec![vec![Square::new(); size]; size]),
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<Square> {
        if (x >= self.size) || (y >= self.size) {
            return None;
        }
        Some(self.squares.borrow()[x][y].clone())
    }

    pub fn random_square(&self) -> (usize, usize) {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..self.size);
        let y = rng.gen_range(0..self.size);
        (x, y)
    }

    pub fn place_ship(
        &self,
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
                    if let Some(existing_ship) = square.ship {
                        return Err(eyre!("{} overlaps with {}", ship, existing_ship));
                    }
                }
            }
        }

        // Actually place the ship
        for i in 0..ship.length() {
            let pos_x = x + i * step_x;
            let pos_y = y + i * step_y;
            self.squares.borrow_mut()[pos_x][pos_y].place_ship(ship);
        }

        Ok(())
    }

    pub fn fire_at(&self, x: usize, y: usize) -> Option<Ship> {
        match self.at(x, y) {
            None => None,
            Some(_) => self.squares.borrow_mut()[x][y].fire(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Square {
    pub ship: Option<Ship>,
    pub hit: bool,
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

    fn fire(&mut self) -> Option<Ship> {
        if self.hit {
            return None;
        }
        self.hit = true;
        self.ship
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_inits_players_and_grids() {
        let game = Game::new(vec!["Alice".to_string(), "Bob".to_string()], 10);

        assert_eq!(game.players.len(), 2);
        assert_eq!(game.players[0], "Alice");
        assert_eq!(game.players[1], "Bob");
        assert_eq!(game.grids.len(), 2);
        assert_eq!(game.grids[0].size, 10);
        assert_eq!(game.grids[1].size, 10);
    }
}
