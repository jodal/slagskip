use std::fmt;

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
    pub(crate) fn step(&self) -> (usize, usize) {
        match self {
            Self::Horizontal => (1, 0),
            Self::Vertical => (0, 1),
        }
    }
}
