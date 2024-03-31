use std::cmp::min;

use crate::game::Point;

#[derive(Debug)]
pub struct Cursor {
    cols: usize,
    rows: usize,
    pub point: Point,
}

impl Cursor {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            point: Point(cols / 2, rows / 2),
        }
    }

    pub fn up(&mut self) {
        self.point = Point(self.point.0, self.point.1.saturating_sub(1));
    }

    pub fn left(&mut self) {
        self.point = Point(self.point.0.saturating_sub(1), self.point.1);
    }

    pub fn down(&mut self) {
        self.point = Point(
            self.point.0,
            min(self.point.1.saturating_add(1), self.rows - 1),
        );
    }

    pub fn right(&mut self) {
        self.point = Point(
            min(self.point.0.saturating_add(1), self.cols - 1),
            self.point.1,
        );
    }
}
