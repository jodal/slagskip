use eyre::{eyre, Result};

use crate::core::player::Player;

use super::{Active, New};

#[derive(Debug)]
pub struct Game<Stage> {
    stage: std::marker::PhantomData<Stage>,
    pub grid_size: usize,
    pub players: Vec<Player<Stage>>,
}

impl Default for Game<Active> {
    fn default() -> Self {
        // Only used for test setup.
        Self {
            stage: std::marker::PhantomData,
            grid_size: 10,
            players: vec![Player::<Active>::default(), Player::<Active>::default()],
        }
    }
}

impl Game<New> {
    pub fn new(grid_size: usize) -> Game<New> {
        Self {
            stage: std::marker::PhantomData,
            grid_size,
            players: vec![],
        }
    }

    pub fn add_player(&mut self, name: &str) -> &Player<New> {
        let player = Player::new(name, self.grid_size);
        self.players.push(player);
        self.players.last().unwrap()
    }

    pub fn is_ready(&self) -> bool {
        self.players.iter().filter(|np| np.is_ready()).count() >= 2
    }

    pub fn start(self) -> Result<Game<Active>> {
        if !self.is_ready() {
            return Err(eyre!("Not enough players are ready to start."));
        }

        let players = self
            .players
            .into_iter()
            .filter_map(|np| np.ready().ok())
            .collect();

        Ok(Game {
            stage: std::marker::PhantomData,
            grid_size: self.grid_size,
            players,
        })
    }
}

impl Game<Active> {
    fn alive_players(&self) -> Vec<&Player<Active>> {
        self.players.iter().filter(|p| p.is_alive()).collect()
    }

    pub fn result(&self) -> Option<GameResult> {
        match self.alive_players()[..] {
            [player] => Some(GameResult::Winner(player)),
            [] => Some(GameResult::Draw),
            _ => None,
        }
    }

    pub fn round(&self) -> Vec<Turn> {
        self.players
            .iter()
            .map(|player| {
                Turn::new(
                    player,
                    self.alive_players()
                        .into_iter()
                        .filter(|p| *p != player)
                        .collect(),
                )
            })
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Turn<'a> {
    pub player: &'a Player<Active>,
    pub opponents: Vec<&'a Player<Active>>,
}

impl<'a> Turn<'a> {
    fn new(player: &'a Player<Active>, opponents: Vec<&'a Player<Active>>) -> Self {
        Self { player, opponents }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum GameResult<'a> {
    Winner(&'a Player<Active>),
    Draw,
}

#[cfg(test)]
mod tests {
    use eyre::Result;

    use super::*;

    #[test]
    fn game_setup() -> Result<()> {
        let mut new_game = Game::new(10);

        let alice = new_game.add_player("Alice");
        assert_eq!(alice.name, "Alice");
        assert_eq!(alice.grid.size, 10);
        alice.place_ships_randomly()?;
        assert!(alice.is_ready());
        assert!(!new_game.is_ready());

        let bob = new_game.add_player("Bob");
        assert_eq!(bob.name, "Bob");
        assert_eq!(bob.grid.size, 10);
        bob.place_ships_randomly()?;
        assert!(bob.is_ready());
        assert!(new_game.is_ready());

        Ok(())
    }

    #[test]
    fn round_pairs_each_player_with_opponents() -> Result<()> {
        let mut new_game = Game::new(10);
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
