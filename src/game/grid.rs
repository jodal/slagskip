use eyre::{eyre, Result};
use rand::{thread_rng, Rng};
use std::cell::RefCell;

use super::{Direction, Ship};

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    pub size: usize,
    points: RefCell<Vec<Vec<Point>>>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        Grid {
            size,
            points: RefCell::new(vec![vec![Point::new(); size]; size]),
        }
    }

    pub(crate) fn at(&self, x: usize, y: usize) -> Option<Point> {
        if (x >= self.size) || (y >= self.size) {
            return None;
        }
        Some(self.points.borrow()[x][y].clone())
    }

    pub fn random_point(&self) -> (usize, usize) {
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
            self.points.borrow_mut()[pos_x][pos_y].place_ship(ship);
        }

        Ok(())
    }

    pub fn fire_at(&self, x: usize, y: usize) -> Option<Ship> {
        match self.at(x, y) {
            None => None,
            Some(_) => self.points.borrow_mut()[x][y].fire(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub ship: Option<Ship>,
    pub hit: bool,
}

impl Point {
    fn new() -> Self {
        Point {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_point() {
        let grid_size = 10;
        let grid = Grid::new(grid_size);

        let (x, y) = grid.random_point();

        assert!(x < grid_size);
        assert!(y < grid_size);
    }

    #[test]
    fn place_ship_horizontal() {
        let grid = Grid::new(10);

        grid.place_ship(Ship::Destroyer, (0, 0), Direction::Horizontal)
            .unwrap();

        assert_eq!(grid.at(0, 0).unwrap().ship, Some(Ship::Destroyer));
        assert_eq!(grid.at(1, 0).unwrap().ship, Some(Ship::Destroyer));
        assert_eq!(grid.at(2, 0).unwrap().ship, None);
    }

    #[test]
    fn place_ship_vertical() {
        let grid = Grid::new(10);

        grid.place_ship(Ship::Destroyer, (1, 1), Direction::Vertical)
            .unwrap();

        assert_eq!(grid.at(1, 1).unwrap().ship, Some(Ship::Destroyer));
        assert_eq!(grid.at(1, 2).unwrap().ship, Some(Ship::Destroyer));
        assert_eq!(grid.at(1, 3).unwrap().ship, None);
    }

    #[test]
    fn place_ship_out_of_bounds() {
        let grid = Grid::new(10);

        // When a destroyer of length two is placed on the last point on a row
        let result = grid.place_ship(Ship::Destroyer, (9, 0), Direction::Horizontal);

        assert!(result.is_err());
    }

    #[test]
    fn place_ship_overlapping_existing_ship() {
        let grid = Grid::new(10);
        // Given a carrier in the first five points: CCCCC.....
        grid.place_ship(Ship::Carrier, (0, 0), Direction::Horizontal)
            .unwrap();

        // When a destroyer is placed overlapping the carrier: CCCCDD....
        let result = grid.place_ship(Ship::Destroyer, (4, 0), Direction::Horizontal);

        assert!(result.is_err());
    }

    #[test]
    fn fire_at() {
        // Given a carrier: CCCCC.....
        let grid = Grid::new(10);
        grid.place_ship(Ship::Carrier, (0, 0), Direction::Horizontal)
            .unwrap();

        // CCCCCx.... is a miss
        assert_eq!(grid.fire_at(5, 0), None);

        // CCCXCx.... is a hit
        assert_eq!(grid.fire_at(3, 0), Some(Ship::Carrier));

        // Another hit in the same spot is a miss as there is no longer anything there
        assert_eq!(grid.fire_at(3, 0), None);
    }
}
