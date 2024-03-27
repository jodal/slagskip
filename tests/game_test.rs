use eyre::Result;
use slagskip::game::{Direction, NewGame, PlayerStatus, Ship};

#[test]
fn one_ship_game() -> Result<()> {
    let mut new_game = NewGame::new(2);
    new_game.add_player("Alice");
    new_game.add_player("Bob");

    {
        let alice = &new_game.players[0];
        let bob = &new_game.players[1];

        assert_eq!(alice.status(), PlayerStatus::SETUP);
        assert_eq!(bob.status(), PlayerStatus::SETUP);

        // Place ships
        alice.place_ship(Ship::Destroyer, (0, 0), Direction::Horizontal)?;
        bob.place_ship(Ship::Destroyer, (1, 0), Direction::Vertical)?;
    }

    let game = new_game.start()?;

    {
        let alice = &game.players[0];
        let bob = &game.players[1];

        assert_eq!(alice.status(), PlayerStatus::PLAYING);
        assert_eq!(bob.status(), PlayerStatus::PLAYING);

        // Let everyone have 2 turns
        for i in 0..2 {
            for turn in game.round() {
                for opponent in turn.opponents.iter() {
                    opponent.fire_at(i, i);
                }
            }
        }

        assert_eq!(alice.grid.to_string(), ["XO", ".x"].join("\n"));
        assert_eq!(bob.grid.to_string(), ["xO", ".X"].join("\n"));

        assert_eq!(alice.status(), PlayerStatus::PLAYING);
        assert_eq!(bob.status(), PlayerStatus::PLAYING);
    }

    Ok(())
}
