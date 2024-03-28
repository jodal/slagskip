mod grid;
mod player;
mod ship;

use eyre::Result;

pub use crate::game::grid::{Grid, Point};
pub use crate::game::player::{ActivePlayer, NewPlayer};
pub use crate::game::ship::{Direction, Ship};

#[derive(Debug)]
pub struct NewGame {
    grid_size: usize,
    pub players: Vec<NewPlayer>,
}

impl NewGame {
    pub fn new(grid_size: usize) -> Self {
        Self {
            grid_size,
            players: vec![],
        }
    }

    pub fn add_player(&mut self, name: &str) -> &NewPlayer {
        let player = NewPlayer::new(name, self.grid_size);
        self.players.push(player);
        self.players.last().unwrap()
    }

    pub fn start(self) -> Result<ActiveGame> {
        let players = self
            .players
            .into_iter()
            .filter_map(|np| np.ready().ok())
            .collect();

        // TODO: Check that at least two players are ready to start

        Ok(ActiveGame { players })
    }
}

#[derive(Debug)]
pub struct ActiveGame {
    pub players: Vec<ActivePlayer>,
}

impl ActiveGame {
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
pub struct Turn<'a> {
    pub player: &'a ActivePlayer,
    pub opponents: Vec<&'a ActivePlayer>,
}

impl<'a> Turn<'a> {
    fn new(player: &'a ActivePlayer, opponents: Vec<&'a ActivePlayer>) -> Self {
        Self { player, opponents }
    }
}

#[cfg(test)]
mod tests {
    use eyre::Result;

    use super::*;

    #[test]
    fn game_setup() {
        let mut new_game = NewGame::new(10);

        let alice = new_game.add_player("Alice");
        assert_eq!(alice.name, "Alice");
        assert_eq!(alice.grid.size, 10);

        let bob = new_game.add_player("Bob");
        assert_eq!(bob.name, "Bob");
        assert_eq!(bob.grid.size, 10);
    }

    #[test]
    fn round_pairs_each_player_with_opponents() -> Result<()> {
        let mut new_game = NewGame::new(10);
        let alice = new_game.add_player("Alice");
        alice.place_ships_randomly()?;
        let bob = new_game.add_player("Bob");
        bob.place_ships_randomly()?;
        let cecil = new_game.add_player("Cecil");
        cecil.place_ships_randomly()?;

        let game = new_game.start()?;
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

        Ok(())
    }
}
