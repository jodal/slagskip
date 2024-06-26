use rand::{thread_rng, Rng};
use strum::IntoEnumIterator;

#[derive(Copy, Clone, Debug, Eq, PartialEq, strum_macros::EnumIter, strum_macros::Display)]
pub enum Ship {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
}

impl Ship {
    pub fn for_grid(grid_size: usize) -> Vec<Ship> {
        Ship::iter().filter(|s| s.length() <= grid_size).collect()
    }

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

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        match rng.gen_bool(0.5) {
            true => Direction::Horizontal,
            false => Direction::Vertical,
        }
    }

    pub fn step(&self) -> (usize, usize) {
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
    fn ships_for_grid() {
        assert_eq!(
            Ship::for_grid(10),
            vec![
                Ship::Carrier,
                Ship::Battleship,
                Ship::Cruiser,
                Ship::Submarine,
                Ship::Destroyer
            ]
        );
        assert_eq!(
            Ship::for_grid(5),
            vec![
                Ship::Carrier,
                Ship::Battleship,
                Ship::Cruiser,
                Ship::Submarine,
                Ship::Destroyer
            ]
        );
        assert_eq!(
            Ship::for_grid(4),
            vec![
                Ship::Battleship,
                Ship::Cruiser,
                Ship::Submarine,
                Ship::Destroyer
            ]
        );
        assert_eq!(
            Ship::for_grid(3),
            vec![Ship::Cruiser, Ship::Submarine, Ship::Destroyer]
        );
        assert_eq!(Ship::for_grid(2), vec![Ship::Destroyer]);
        assert_eq!(Ship::for_grid(1), vec![]);
    }
}
