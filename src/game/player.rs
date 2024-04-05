use std::cell::RefCell;

use eyre::{eyre, Result};

use super::{grid::Fire, Active, Direction, Grid, New, Point, Ship};

#[derive(Debug, Eq, PartialEq)]
pub struct Player<Stage> {
    stage: std::marker::PhantomData<Stage>,
    pub name: String,
    pub to_place: RefCell<Vec<Ship>>,
    pub grid: Grid,
}

impl Default for Player<Active> {
    fn default() -> Self {
        // Only used for test setup.
        Self {
            stage: std::marker::PhantomData::<Active>,
            name: "Default".into(),
            to_place: RefCell::new(vec![]),
            grid: Grid::new(10),
        }
    }
}

impl Player<New> {
    pub fn new(name: &str, grid_size: usize) -> Player<New> {
        Self {
            stage: std::marker::PhantomData::<New>,
            name: name.to_string(),
            to_place: RefCell::new(Ship::for_grid(grid_size)),
            grid: Grid::new(grid_size),
        }
    }

    pub fn get_ship_to_place(&self) -> Option<Ship> {
        match self.to_place.borrow().first() {
            Some(ship) => Some(ship.clone()),
            None => None,
        }
    }

    fn get_place_ship_index(&self, ship: Ship) -> Result<usize> {
        self.to_place
            .borrow()
            .iter()
            .position(|s| *s == ship)
            .ok_or_else(|| {
                eyre!(
                    "{} is not to be placed. Expected one of {:?}",
                    ship,
                    self.to_place.borrow()
                )
            })
    }

    fn remove_ship_to_place(&self, ship: Ship) -> Result<()> {
        let index = self.get_place_ship_index(ship)?;
        self.to_place.borrow_mut().remove(index);
        Ok(())
    }

    pub fn place_ship(&self, ship: Ship, point: Point, direction: Direction) -> Result<()> {
        let (step_x, step_y) = direction.step();

        // Check that ship is to be placed
        self.get_place_ship_index(ship)?;

        // Check the placement
        for i in 0..ship.length() {
            let point_i = Point(point.0 + i * step_x, point.1 + i * step_y);
            match self.grid.at(point_i) {
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
            if let Some(cell) = self
                .grid
                .at(Point(point.0 + i * step_x, point.1 + i * step_y))
            {
                cell.place_ship(ship);
            }
        }
        self.remove_ship_to_place(ship)?;

        Ok(())
    }

    pub fn place_ships_randomly(&self) -> Result<()> {
        while let Some(ship) = self.get_ship_to_place() {
            loop {
                let point = self.grid.random_point();
                match self.place_ship(ship, point, Direction::random()) {
                    Ok(_) => {
                        break;
                    }
                    Err(_) => {
                        // Try again
                    }
                }
            }
        }
        Ok(())
    }

    pub fn is_ready(&self) -> bool {
        self.to_place.borrow().is_empty()
    }

    pub fn ready(self) -> Result<Player<Active>> {
        if !self.is_ready() {
            return Err(eyre!(
                "Player {} has not placed all ships: {:?}",
                self.name,
                self.to_place
            ));
        }

        Ok(Player {
            stage: std::marker::PhantomData::<Active>,
            name: self.name,
            to_place: self.to_place,
            grid: self.grid,
        })
    }
}

impl Player<Active> {
    pub fn fire_at(&self, point: Point) -> Option<Fire> {
        self.grid.at(point).and_then(|cell| cell.fire())
    }

    pub fn fire_at_random(&self) -> Option<(Point, Fire)> {
        let max_attempts = self.grid.size * self.grid.size;
        for _ in 0..max_attempts {
            let (point, cell) = self.grid.random_cell();
            match cell.is_hit() {
                true => {
                    // Select a new cell to hit
                }
                false => {
                    return Some((point, cell.fire().unwrap()));
                }
            }
        }
        None
    }

    pub fn num_ships_total(&self) -> usize {
        self.grid.cells().filter(|p| p.has_ship().is_some()).count()
    }

    pub fn num_ships_alive(&self) -> usize {
        self.grid
            .cells()
            .filter(|p| p.has_ship().is_some() && !p.is_hit())
            .count()
    }

    pub fn is_alive(&self) -> bool {
        self.num_ships_alive() > 0
    }
}

#[cfg(test)]
mod tests {
    use eyre::Result;

    use crate::game::{Direction, Ship};

    use super::*;

    #[test]
    fn place_ship_horizontal() -> Result<()> {
        let player = Player::new("Alice", 3);

        player.place_ship(Ship::Destroyer, Point(0, 0), Direction::Horizontal)?;

        assert_eq!(player.grid.to_string(), ["OO.", "...", "..."].join("\n"));
        Ok(())
    }

    #[test]
    fn place_ship_vertical() -> Result<()> {
        let player = Player::new("Alice", 3);

        player.place_ship(Ship::Destroyer, Point(1, 1), Direction::Vertical)?;

        assert_eq!(player.grid.to_string(), ["...", ".O.", ".O."].join("\n"));
        Ok(())
    }

    #[test]
    fn place_ship_out_of_bounds() -> Result<()> {
        let player = Player::new("Alice", 10);

        // When a destroyer of length two is placed on the last cell on a row
        let result = player.place_ship(Ship::Destroyer, Point(9, 0), Direction::Horizontal);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn place_ship_overlapping_existing_ship() -> Result<()> {
        let player = Player::new("Alice", 10);
        // Given a carrier in the first five cells: CCCCC.....
        player.place_ship(Ship::Carrier, Point(0, 0), Direction::Horizontal)?;

        // When a destroyer is placed overlapping the carrier: CCCCDD....
        let result = player.place_ship(Ship::Destroyer, Point(4, 0), Direction::Horizontal);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn place_same_ship_twice() -> Result<()> {
        let player = Player::new("Alice", 10);
        player.place_ship(Ship::Destroyer, Point(0, 0), Direction::Horizontal)?;

        let result = player.place_ship(Ship::Destroyer, Point(0, 1), Direction::Horizontal);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn fire_at() -> Result<()> {
        // Given a carrier: CCCCC.....
        let new_player = Player::new("Alice", 2);
        new_player.place_ship(Ship::Destroyer, Point(0, 0), Direction::Horizontal)?;
        let player = new_player.ready()?;

        // CC/x. is a miss
        assert_eq!(player.fire_at(Point(0, 1)), Some(Fire::Miss));

        // XC/x. is a hit
        assert_eq!(player.fire_at(Point(0, 0)), Some(Fire::Hit));

        // Another hit in the same spot does not count as a move
        assert_eq!(player.fire_at(Point(0, 0)), None);
        Ok(())
    }

    #[test]
    fn status_checks_if_any_ships_remain() -> Result<()> {
        let new_player = Player::new("Alice", 3);
        new_player.place_ship(Ship::Submarine, Point(0, 0), Direction::Horizontal)?;

        // There are more ships to place
        assert!(!new_player.is_ready());

        new_player.place_ship(Ship::Cruiser, Point(0, 1), Direction::Horizontal)?;
        new_player.place_ship(Ship::Destroyer, Point(0, 2), Direction::Horizontal)?;

        // All ships have been placed
        let player = new_player.ready()?;

        // A miss
        player.fire_at(Point(2, 2));
        assert!(player.is_alive());

        // Sink Submarine
        player.fire_at(Point(0, 0));
        player.fire_at(Point(1, 0));
        player.fire_at(Point(2, 0));
        assert!(player.is_alive());

        // Sink Cruiser
        player.fire_at(Point(0, 1));
        player.fire_at(Point(1, 1));
        player.fire_at(Point(2, 1));
        assert!(player.is_alive());

        // Sink Destroyer
        player.fire_at(Point(0, 2));
        player.fire_at(Point(1, 2));
        assert!(!player.is_alive());

        Ok(())
    }
}
