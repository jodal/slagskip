use rand::{thread_rng, Rng};
use std::{cell::RefCell, fmt};

use super::Ship;

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    pub size: usize,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub(crate) fn new(size: usize) -> Self {
        Grid {
            size,
            cells: vec![vec![Cell::new(); size]; size],
        }
    }

    pub(crate) fn cells(&self) -> CellIter {
        CellIter::new(&self.cells)
    }

    pub(crate) fn at(&self, point: Point) -> Option<&Cell> {
        if (point.0 >= self.size) || (point.1 >= self.size) {
            return None;
        }
        Some(&self.cells[point.0][point.1])
    }

    pub(crate) fn random_point(&self) -> Point {
        let mut rng = thread_rng();
        Point(rng.gen_range(0..self.size), rng.gen_range(0..self.size))
    }

    pub(crate) fn random_cell(&self) -> (Point, &Cell) {
        let point = self.random_point();
        (point, self.at(point).unwrap())
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::with_capacity((self.size + 1) * self.size - 1);
        for y in 0..self.size {
            for x in 0..self.size {
                if let Some(cell) = self.at(Point(x, y)) {
                    match (cell.has_ship(), cell.is_hit()) {
                        (Some(_ship), false) => buf.push('O'),
                        (Some(_ship), true) => buf.push('X'),
                        (None, false) => buf.push('.'),
                        (None, true) => buf.push('_'),
                    }
                }
            }
            if y < self.size - 1 {
                buf.push('\n');
            }
        }
        write!(f, "{}", buf)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Cell {
    ship: RefCell<Option<Ship>>,
    hit: RefCell<bool>,
}

impl Cell {
    fn new() -> Self {
        Cell {
            ship: RefCell::new(None),
            hit: RefCell::new(false),
        }
    }

    pub(crate) fn has_ship(&self) -> Option<Ship> {
        self.ship.borrow().to_owned()
    }

    pub(crate) fn is_hit(&self) -> bool {
        *self.hit.borrow()
    }

    pub(crate) fn place_ship(&self, ship: Ship) {
        *self.ship.borrow_mut() = Some(ship);
    }

    pub(crate) fn fire(&self) -> Fire {
        if self.is_hit() {
            return Fire::Miss;
        }
        *self.hit.borrow_mut() = true;
        match self.has_ship() {
            Some(_ship) => {
                // TODO Detect if entire ship is hit, and if so return Sunk(ship)
                return Fire::Hit;
            }
            None => {
                return Fire::Miss;
            }
        }
    }
}

pub(crate) struct CellIter<'a> {
    cells: &'a Vec<Vec<Cell>>,
    row_index: usize,
    column_index: usize,
}

impl<'a> CellIter<'a> {
    fn new(cells: &'a Vec<Vec<Cell>>) -> Self {
        Self {
            cells,
            row_index: 0,
            column_index: 0,
        }
    }
}

impl<'a> Iterator for CellIter<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        // Check if the current row is exhausted
        while self.row_index < self.cells.len()
            && self.column_index >= self.cells[self.row_index].len()
        {
            self.row_index += 1;
            self.column_index = 0;
        }

        // Check if all rows are exhausted
        if self.row_index >= self.cells.len() {
            return None;
        }

        // Get the current item and move the iterator forward
        let item = &self.cells[self.row_index][self.column_index];
        self.column_index += 1;
        Some(item)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point(pub usize, pub usize);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", (65u8 + self.0 as u8) as char, self.1 + 1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fire {
    Miss,
    Hit,
    Sunk(Ship),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cells_iter() {
        assert_eq!(Grid::new(3).cells().count(), 9);
        assert_eq!(Grid::new(5).cells().count(), 25);
    }

    #[test]
    fn random_cell() {
        let grid_size = 10;
        let grid = Grid::new(grid_size);

        let (point, cell) = grid.random_cell();

        assert!(point.0 < grid_size);
        assert!(point.1 < grid_size);
        assert_eq!(grid.at(point).unwrap(), cell);
    }

    #[test]
    fn point_format() {
        assert_eq!(Point(0, 0).to_string(), "A1");
        assert_eq!(Point(0, 1).to_string(), "A2");
        assert_eq!(Point(1, 0).to_string(), "B1");
        assert_eq!(Point(2, 4).to_string(), "C5");
    }
}
