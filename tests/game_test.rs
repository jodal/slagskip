use eyre::Result;
use slagskip::core::{Direction, Game, GameResult, Point, Ship};

#[test]
fn one_ship_game() -> Result<()> {
    let mut game = Game::new(2);
    game.add_player("Alice");
    game.add_player("Bob");

    {
        let alice = &game.players[0];
        let bob = &game.players[1];

        // Place ships
        alice.place_ship(Ship::Destroyer, Point(0, 0), Direction::Horizontal)?;
        bob.place_ship(Ship::Destroyer, Point(1, 0), Direction::Vertical)?;
    }

    let game = game.start()?;

    {
        let alice = &game.players[0];
        let bob = &game.players[1];

        assert!(alice.is_alive());
        assert!(bob.is_alive());

        // Let everyone have 2 turns
        for i in 0..2 {
            for turn in game.round() {
                for opponent in turn.opponents.iter() {
                    opponent.fire_at(Point(0, i));
                }
            }
        }

        assert_eq!(alice.grid.to_string(), ["XO", "_."].join("\n"));
        assert_eq!(bob.grid.to_string(), ["_O", "_O"].join("\n"));
        assert!(alice.is_alive());
        assert!(bob.is_alive());
        assert!(game.result().is_none());

        // Let everyone have another turn
        for turn in game.round() {
            for opponent in turn.opponents.iter() {
                opponent.fire_at(Point(1, 0));
            }
        }

        assert_eq!(alice.grid.to_string(), ["XX", "_."].join("\n"));
        assert_eq!(bob.grid.to_string(), ["_X", "_O"].join("\n"));
        assert!(!alice.is_alive());
        assert!(bob.is_alive());
        match game.result().unwrap() {
            GameResult::Winner(winner) => {
                assert_eq!(winner.name, "Bob");
            }
            GameResult::Draw => {
                panic!("Game should end with a winner, but ended in a tie.")
            }
        }
    }

    Ok(())
}
