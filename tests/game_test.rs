use eyre::Result;
use slagskip::game::{Direction, Game, PlayerStatus, Ship};

#[test]
fn one_ship_game() -> Result<()> {
    let game = Game::new(vec!["Alice".to_string(), "Bob".to_string()], 2);

    let alice = &game.players[0];
    assert_eq!(alice.name, "Alice");
    let bob = &game.players[1];
    assert_eq!(bob.name, "Bob");

    assert_eq!(alice.status(), PlayerStatus::SETUP);
    assert_eq!(bob.status(), PlayerStatus::SETUP);

    // Place ships
    alice
        .grid
        .place_ship(Ship::Destroyer, (0, 0), Direction::Horizontal)?;
    bob.grid
        .place_ship(Ship::Destroyer, (1, 0), Direction::Vertical)?;

    assert_eq!(alice.status(), PlayerStatus::PLAYING);
    assert_eq!(bob.status(), PlayerStatus::PLAYING);

    // Let everyone have 2 turns
    for i in 0..2 {
        for turn in game.round() {
            for opponent in turn.opponents.iter() {
                opponent.grid.fire_at(i, i);
            }
        }
    }

    assert_eq!(alice.grid.to_string(), ["XO", ".x"].join("\n"));
    assert_eq!(bob.grid.to_string(), ["xO", ".X"].join("\n"));

    assert_eq!(alice.status(), PlayerStatus::PLAYING);
    assert_eq!(bob.status(), PlayerStatus::PLAYING);

    Ok(())
}
