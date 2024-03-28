use std::cell::RefCell;

use eyre::{eyre, Result};

use super::{Direction, Grid, Point, Ship};

#[derive(Debug, Eq, PartialEq)]
pub struct NewPlayer {
    pub name: String,
    pub to_place: RefCell<Vec<Ship>>,
    pub grid: Grid,
}

impl NewPlayer {
    pub fn new(name: &str, grid_size: usize) -> Self {
        Self {
            name: name.to_string(),
            to_place: RefCell::new(Ship::for_grid(grid_size)),
            grid: Grid::new(grid_size),
        }
    }

    pub fn place_ship(
        &self,
        ship: Ship,
        (x, y): (usize, usize),
        direction: Direction,
    ) -> Result<()> {
        // Remove ship from self.to_place
        let mut to_place = self.to_place.borrow_mut();
        if let Some(index) = to_place.iter().position(|s| *s == ship) {
            to_place.remove(index);
        } else {
            return Err(eyre!(
                "Tried placing {}; expected one of {:?}.",
                ship,
                to_place
            ));
        }

        let (step_x, step_y) = direction.step();

        // Validate the placement
        for i in 0..ship.length() {
            let pos_x = x + i * step_x;
            let pos_y = y + i * step_y;

            match self.grid.at(pos_x, pos_y) {
                None => return Err(eyre!("{} is out of bounds", ship)),
                Some(cell) => {
                    if let Some(existing_ship) = cell.has_ship() {
                        return Err(eyre!("{} overlaps with {}", ship, existing_ship));
                    }
                }
            }
        }

        // Actually place the ship
        for i in 0..ship.length() {
            if let Some(cell) = self.grid.at(x + i * step_x, y + i * step_y) {
                cell.place_ship(ship);
            }
        }

        Ok(())
    }

    pub fn place_ships_randomly(&self) -> Result<()> {
        // TODO Random placement ot make this more interesting
        self.place_ship(Ship::Carrier, (1, 1), Direction::Horizontal)?;
        self.place_ship(Ship::Battleship, (8, 2), Direction::Vertical)?;
        self.place_ship(Ship::Cruiser, (3, 7), Direction::Vertical)?;
        self.place_ship(Ship::Submarine, (0, 4), Direction::Horizontal)?;
        self.place_ship(Ship::Destroyer, (5, 6), Direction::Horizontal)?;
        Ok(())
    }

    pub fn is_ready(&self) -> bool {
        self.to_place.borrow().is_empty()
    }

    pub fn ready(self) -> Result<ActivePlayer> {
        if !self.is_ready() {
            return Err(eyre!(
                "Player {} has not placed all ships: {:?}",
                self.name,
                self.to_place
            ));
        }

        Ok(ActivePlayer {
            name: self.name,
            grid: self.grid,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ActivePlayer {
    pub name: String,
    pub grid: Grid,
}

impl ActivePlayer {
    pub fn fire_at(&self, x: usize, y: usize) -> Option<Ship> {
        self.grid.at(x, y).and_then(|cell| cell.fire())
    }

    pub fn fire_at_random(&self) -> Option<(Point, Option<Ship>)> {
        let max_attempts = self.grid.size * self.grid.size;
        for _ in 0..max_attempts {
            let (point, cell) = self.grid.random_cell();
            match cell.is_hit() {
                true => {
                    // Select a new cell to hit
                }
                false => {
                    return Some((point, cell.fire()));
                }
            }
        }
        None
    }

    pub fn is_alive(&self) -> bool {
        let num_alive = self
            .grid
            .cells()
            .filter(|p| p.has_ship().is_some() && !p.is_hit())
            .count();
        num_alive > 0
    }
}

#[cfg(test)]
mod tests {
    use eyre::Result;

    use crate::game::{Direction, Ship};

    use super::*;

    #[test]
    fn place_ship_horizontal() -> Result<()> {
        let player = NewPlayer::new("Alice", 3);

        player.place_ship(Ship::Destroyer, (0, 0), Direction::Horizontal)?;

        assert_eq!(player.grid.to_string(), ["OO.", "...", "..."].join("\n"));
        Ok(())
    }

    #[test]
    fn place_ship_vertical() -> Result<()> {
        let player = NewPlayer::new("Alice", 3);

        player.place_ship(Ship::Destroyer, (1, 1), Direction::Vertical)?;

        assert_eq!(player.grid.to_string(), ["...", ".O.", ".O."].join("\n"));
        Ok(())
    }

    #[test]
    fn place_ship_out_of_bounds() -> Result<()> {
        let player = NewPlayer::new("Alice", 10);

        // When a destroyer of length two is placed on the last cell on a row
        let result = player.place_ship(Ship::Destroyer, (9, 0), Direction::Horizontal);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn place_ship_overlapping_existing_ship() -> Result<()> {
        let player = NewPlayer::new("Alice", 10);
        // Given a carrier in the first five cells: CCCCC.....
        player.place_ship(Ship::Carrier, (0, 0), Direction::Horizontal)?;

        // When a destroyer is placed overlapping the carrier: CCCCDD....
        let result = player.place_ship(Ship::Destroyer, (4, 0), Direction::Horizontal);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn place_same_ship_twice() -> Result<()> {
        let player = NewPlayer::new("Alice", 10);
        player.place_ship(Ship::Destroyer, (0, 0), Direction::Horizontal)?;

        let result = player.place_ship(Ship::Destroyer, (0, 1), Direction::Horizontal);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn fire_at() -> Result<()> {
        // Given a carrier: CCCCC.....
        let new_player = NewPlayer::new("Alice", 2);
        new_player.place_ship(Ship::Destroyer, (0, 0), Direction::Horizontal)?;
        let player = new_player.ready()?;

        // CCx is a miss
        assert_eq!(player.fire_at(2, 0), None);

        // XCx is a hit
        assert_eq!(player.fire_at(0, 0), Some(Ship::Destroyer));

        // Another hit in the same spot is a miss as there is no longer anything there
        assert_eq!(player.fire_at(0, 0), None);
        Ok(())
    }

    #[test]
    fn status_checks_if_any_ships_remain() -> Result<()> {
        let new_player = NewPlayer::new("Alice", 3);
        new_player.place_ship(Ship::Submarine, (0, 0), Direction::Horizontal)?;

        // There are more ships to place
        assert!(!new_player.is_ready());

        new_player.place_ship(Ship::Cruiser, (0, 1), Direction::Horizontal)?;
        new_player.place_ship(Ship::Destroyer, (0, 2), Direction::Horizontal)?;

        // All ships have been placed
        let player = new_player.ready()?;

        // A miss
        player.fire_at(2, 2);
        assert!(player.is_alive());

        // Sink Submarine
        player.fire_at(0, 0);
        player.fire_at(1, 0);
        player.fire_at(2, 0);
        assert!(player.is_alive());

        // Sink Cruiser
        player.fire_at(0, 1);
        player.fire_at(1, 1);
        player.fire_at(2, 1);
        assert!(player.is_alive());

        // Sink Destroyer
        player.fire_at(0, 2);
        player.fire_at(1, 2);
        assert!(!player.is_alive());

        Ok(())
    }
}
