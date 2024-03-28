use eyre::Result;
use slagskip::game::{Direction, NewGame, Ship};

#[test]
fn one_ship_game() -> Result<()> {
    let mut new_game = NewGame::new(2);
    new_game.add_player("Alice");
    new_game.add_player("Bob");

    {
        let alice = &new_game.players[0];
        let bob = &new_game.players[1];

        // Place ships
        alice.place_ship(Ship::Destroyer, (0, 0), Direction::Horizontal)?;
        bob.place_ship(Ship::Destroyer, (1, 0), Direction::Vertical)?;
    }

    let game = new_game.start()?;

    {
        let alice = &game.players[0];
        let bob = &game.players[1];

        assert!(alice.is_alive());
        assert!(bob.is_alive());

        // Let everyone have 2 turns
        for i in 0..2 {
            for turn in game.round() {
                for opponent in turn.opponents.iter() {
                    opponent.fire_at(i, i);
                }
            }
        }

        assert_eq!(alice.grid.to_string(), ["XO", "._"].join("\n"));
        assert_eq!(bob.grid.to_string(), ["_O", ".X"].join("\n"));

        assert!(alice.is_alive());
        assert!(bob.is_alive());

        assert!(game.winner().is_none());
    }

    Ok(())
}
