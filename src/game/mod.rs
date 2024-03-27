mod grid;
mod ship;

pub use crate::game::grid::{Grid, Point};
pub use crate::game::ship::{Direction, Ship};

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
}

impl Game {
    pub fn new(player_names: Vec<String>, grid_size: usize) -> Self {
        Self {
            players: player_names
                .iter()
                .map(|name| Player::new(name, grid_size))
                .collect(),
        }
    }

    pub fn round(&self) -> Vec<Turn> {
        self.players
            .iter()
            .map(|player| {
                Turn::new(
                    player,
                    self.players.iter().filter(|&p| p != player).collect(),
                )
            })
            .collect()
    }
}

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

    pub fn status(&self) -> PlayerStatus {
        let num_placed = self
            .grid
            .points()
            .filter(|p| p.has_ship().is_some())
            .count();
        let num_hit = self
            .grid
            .points()
            .filter(|p| p.has_ship().is_some() && p.is_hit())
            .count();

        if num_placed == 0 {
            return PlayerStatus::SETUP;
        }
        if num_hit == num_placed {
            return PlayerStatus::DEAD;
        }
        return PlayerStatus::PLAYING;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlayerStatus {
    SETUP,
    PLAYING,
    DEAD,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Turn<'a> {
    pub player: &'a Player,
    pub opponents: Vec<&'a Player>,
}

impl<'a> Turn<'a> {
    pub fn new(player: &'a Player, opponents: Vec<&'a Player>) -> Self {
        Self { player, opponents }
    }
}

#[cfg(test)]
mod tests {
    use eyre::Result;

    use super::*;

    #[test]
    fn new_inits_players_and_grids() {
        let game = Game::new(vec!["Alice".to_string(), "Bob".to_string()], 10);

        assert_eq!(game.players.len(), 2);
        assert_eq!(game.players[0].name, "Alice");
        assert_eq!(game.players[0].grid.size, 10);
        assert_eq!(game.players[1].name, "Bob");
        assert_eq!(game.players[1].grid.size, 10);
    }

    #[test]
    fn round_pairs_each_player_with_opponents() {
        let game = Game::new(
            vec!["Alice".to_string(), "Bob".to_string(), "Cecil".to_string()],
            10,
        );

        let turns = game.round();

        assert_eq!(turns.len(), game.players.len());
        assert_eq!(*turns[0].player, game.players[0]);
        assert_eq!(*turns[0].opponents[0], game.players[1]);
        assert_eq!(*turns[0].opponents[1], game.players[2]);
        assert_eq!(*turns[1].player, game.players[1]);
        assert_eq!(*turns[1].opponents[0], game.players[0]);
        assert_eq!(*turns[1].opponents[1], game.players[2]);
        assert_eq!(*turns[2].player, game.players[2]);
        assert_eq!(*turns[2].opponents[0], game.players[0]);
        assert_eq!(*turns[2].opponents[1], game.players[1]);
    }

    #[test]
    fn status_checks_if_any_ships_remain() -> Result<()> {
        let player = Player::new("Alice", 3);
        assert_eq!(player.status(), PlayerStatus::SETUP);

        player
            .grid
            .place_ship(Ship::Submarine, (0, 0), Direction::Horizontal)?;
        // TODO Keep track of how many ships should be placed, so that we can stay in SETUP until all are placed.
        // assert_eq!(player.status(), PlayerStatus::SETUP);

        assert_eq!(player.status(), PlayerStatus::PLAYING);

        // A miss
        player.grid.fire_at(1, 0);
        assert_eq!(player.status(), PlayerStatus::PLAYING);

        // One hit
        player.grid.fire_at(0, 0);
        assert_eq!(player.status(), PlayerStatus::PLAYING);

        // Two hits
        player.grid.fire_at(1, 0);
        assert_eq!(player.status(), PlayerStatus::PLAYING);

        // Three hits, the single ship is sunken
        player.grid.fire_at(2, 0);
        assert_eq!(player.status(), PlayerStatus::DEAD);

        Ok(())
    }
}
