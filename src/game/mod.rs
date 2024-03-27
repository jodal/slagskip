mod grid;
mod ship;

pub use crate::game::grid::{Grid, Square};
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
                .map(|name| Player {
                    name: name.to_string(),
                    grid: Grid::new(grid_size),
                })
                .collect(),
        }
    }

    pub fn round(&self) -> Vec<Turn> {
        self.players
            .iter()
            .map(|player| Turn {
                player,
                opponents: self.players.iter().filter(|&p| p != player).collect(),
            })
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Player {
    pub name: String,
    pub grid: Grid,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Turn<'a> {
    pub player: &'a Player,
    pub opponents: Vec<&'a Player>,
}

#[cfg(test)]
mod tests {
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
}
