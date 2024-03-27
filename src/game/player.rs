use eyre::Result;

use super::{Direction, Grid, Ship};

#[derive(Debug, Eq, PartialEq)]
pub struct Player {
    pub name: String,
    pub grid: Grid,
}

impl Player {
    pub fn new(name: &str, grid_size: usize) -> Self {
        Self {
            name: name.to_string(),
            grid: Grid::new(grid_size),
        }
    }

    pub fn place_ships_randomly(&self) -> Result<()> {
        // TODO Random placement ot make this more interesting
        self.grid
            .place_ship(Ship::Carrier, (1, 1), Direction::Horizontal)?;
        self.grid
            .place_ship(Ship::Battleship, (8, 2), Direction::Vertical)?;
        self.grid
            .place_ship(Ship::Cruiser, (3, 7), Direction::Vertical)?;
        self.grid
            .place_ship(Ship::Submarine, (0, 4), Direction::Horizontal)?;
        self.grid
            .place_ship(Ship::Destroyer, (5, 6), Direction::Horizontal)?;
        Ok(())
    }

    pub fn status(&self) -> PlayerStatus {
        if self.grid.to_place.borrow().len() != 0 {
            return PlayerStatus::SETUP;
        }

        let num_alive = self
            .grid
            .cells()
            .filter(|p| p.has_ship().is_some() && !p.is_hit())
            .count();

        if num_alive > 0 {
            return PlayerStatus::PLAYING;
        }
        return PlayerStatus::DEAD;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlayerStatus {
    SETUP,
    PLAYING,
    DEAD,
}
#[cfg(test)]
mod tests {
    use eyre::Result;

    use crate::game::{Direction, Ship};

    use super::*;

    #[test]
    fn status_checks_if_any_ships_remain() -> Result<()> {
        let player = Player::new("Alice", 3);
        assert_eq!(player.status(), PlayerStatus::SETUP);

        player
            .grid
            .place_ship(Ship::Submarine, (0, 0), Direction::Horizontal)?;

        // There are more ships to place
        assert_eq!(player.status(), PlayerStatus::SETUP);

        player
            .grid
            .place_ship(Ship::Cruiser, (0, 1), Direction::Horizontal)?;
        player
            .grid
            .place_ship(Ship::Destroyer, (0, 2), Direction::Horizontal)?;

        // All ships have been placed
        assert_eq!(player.status(), PlayerStatus::PLAYING);

        // A miss
        player.grid.fire_at(2, 2);
        assert_eq!(player.status(), PlayerStatus::PLAYING);

        // Sink Submarine
        player.grid.fire_at(0, 0);
        player.grid.fire_at(1, 0);
        player.grid.fire_at(2, 0);
        assert_eq!(player.status(), PlayerStatus::PLAYING);

        // Sink Cruiser
        player.grid.fire_at(0, 1);
        player.grid.fire_at(1, 1);
        player.grid.fire_at(2, 1);
        assert_eq!(player.status(), PlayerStatus::PLAYING);

        // Sink Destroyer
        player.grid.fire_at(0, 2);
        player.grid.fire_at(1, 2);
        assert_eq!(player.status(), PlayerStatus::DEAD);

        Ok(())
    }
}
