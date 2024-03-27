use eyre::{eyre, Result};
use rand::{thread_rng, Rng};
use std::cell::RefCell;

use super::{Direction, Ship};

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    pub size: usize,
    pub points: RefCell<Vec<Vec<Point>>>,
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
