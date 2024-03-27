use eyre::{eyre, Result};
use rand::{thread_rng, Rng};
use std::cell::RefCell;

use super::{Direction, Ship};

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    pub size: usize,
    to_place: RefCell<Vec<Ship>>,
    points: Vec<Vec<Point>>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        Grid {
            size,
            to_place: RefCell::new(Ship::for_grid(size)),
            points: vec![vec![Point::new(); size]; size],
        }
    }

    pub fn points(&self) -> PointIter {
        PointIter::new(&self.points)
    }

    pub(crate) fn at(&self, x: usize, y: usize) -> Option<&Point> {
        if (x >= self.size) || (y >= self.size) {
            return None;
        }
        Some(&self.points[x][y])
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

            match self.at(pos_x, pos_y) {
                None => return Err(eyre!("{} is out of bounds", ship)),
                Some(square) => {
                    if let Some(existing_ship) = square.has_ship() {
                        return Err(eyre!("{} overlaps with {}", ship, existing_ship));
                    }
                }
            }
        }

        // Actually place the ship
        for i in 0..ship.length() {
            self.points[x + i * step_x][y + i * step_y].place_ship(ship);
        }

        Ok(())
    }

    pub fn fire_at(&self, x: usize, y: usize) -> Option<Ship> {
        match self.at(x, y) {
            None => None,
            Some(_) => self.points[x][y].fire(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity((self.size + 1) * self.size - 1);
        for y in 0..self.size {
            for x in 0..self.size {
                if let Some(point) = self.at(x, y) {
                    match (point.has_ship(), point.is_hit()) {
                        (Some(_ship), false) => result.push('O'),
                        (Some(_ship), true) => result.push('X'),
                        (None, false) => result.push('.'),
                        (None, true) => result.push('x'),
                    }
                }
            }
            if y < self.size - 1 {
                result.push('\n');
            }
        }
        result
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Point {
    ship: RefCell<Option<Ship>>,
    hit: RefCell<bool>,
}

impl Point {
    fn new() -> Self {
        Point {
            ship: RefCell::new(None),
            hit: RefCell::new(false),
        }
    }

    pub fn has_ship(&self) -> Option<Ship> {
        self.ship.borrow().clone()
    }

    pub fn is_hit(&self) -> bool {
        *self.hit.borrow()
    }

    fn place_ship(&self, ship: Ship) {
        *self.ship.borrow_mut() = Some(ship);
    }

    fn fire(&self) -> Option<Ship> {
        if self.is_hit() {
            return None;
        }
        *self.hit.borrow_mut() = true;
        self.has_ship()
    }
}

pub struct PointIter<'a> {
    points: &'a Vec<Vec<Point>>,
    row_index: usize,
    column_index: usize,
}

impl<'a> PointIter<'a> {
    fn new(points: &'a Vec<Vec<Point>>) -> Self {
        Self {
            points,
            row_index: 0,
            column_index: 0,
        }
    }
}

impl<'a> Iterator for PointIter<'a> {
    type Item = &'a Point;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if the current row is exhausted
        while self.row_index < self.points.len()
            && self.column_index >= self.points[self.row_index].len()
        {
            self.row_index += 1;
            self.column_index = 0;
        }

        // Check if all rows are exhausted
        if self.row_index >= self.points.len() {
            return None;
        }

        // Get the current item and move the iterator forward
        let item = &self.points[self.row_index][self.column_index];
        self.column_index += 1;
        Some(item)
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
        let grid = Grid::new(3);

        grid.place_ship(Ship::Destroyer, (0, 0), Direction::Horizontal)
            .unwrap();

        assert_eq!(grid.to_string(), ["OO.", "...", "..."].join("\n"));
    }

    #[test]
    fn place_ship_vertical() {
        let grid = Grid::new(3);

        grid.place_ship(Ship::Destroyer, (1, 1), Direction::Vertical)
            .unwrap();

        assert_eq!(grid.to_string(), ["...", ".O.", ".O."].join("\n"));
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
    fn place_same_ship_twice() {
        let grid = Grid::new(3);

        grid.place_ship(Ship::Destroyer, (0, 0), Direction::Horizontal)
            .unwrap();

        let result = grid.place_ship(Ship::Destroyer, (0, 1), Direction::Horizontal);

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
