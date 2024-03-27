use rand::{thread_rng, Rng};
use std::cell::RefCell;

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

    pub(crate) fn at(&self, x: usize, y: usize) -> Option<&Cell> {
        if (x >= self.size) || (y >= self.size) {
            return None;
        }
        Some(&self.cells[x][y])
    }

    pub fn random_point(&self) -> (usize, usize) {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..self.size);
        let y = rng.gen_range(0..self.size);
        (x, y)
    }

    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity((self.size + 1) * self.size - 1);
        for y in 0..self.size {
            for x in 0..self.size {
                if let Some(cell) = self.at(x, y) {
                    match (cell.has_ship(), cell.is_hit()) {
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
        self.ship.borrow().clone()
    }

    pub(crate) fn is_hit(&self) -> bool {
        *self.hit.borrow()
    }

    pub(crate) fn place_ship(&self, ship: Ship) {
        *self.ship.borrow_mut() = Some(ship);
    }

    pub(crate) fn fire(&self) -> Option<Ship> {
        if self.is_hit() {
            return None;
        }
        *self.hit.borrow_mut() = true;
        self.has_ship()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cells_iter() {
        assert_eq!(Grid::new(3).cells().count(), 9);
        assert_eq!(Grid::new(5).cells().count(), 25);
    }

    #[test]
    fn random_point() {
        let grid_size = 10;
        let grid = Grid::new(grid_size);

        let (x, y) = grid.random_point();

        assert!(x < grid_size);
        assert!(y < grid_size);
    }
}
