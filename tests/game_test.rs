use eyre::Result;
use slagskip::game::{Direction, Game, Ship};

#[test]
fn one_ship_game() -> Result<()> {
    let game = Game::new(vec!["Alice".to_string(), "Bob".to_string()], 5);

    let alice = &game.players[0];
    assert_eq!(alice.name, "Alice");
    let bob = &game.players[1];
    assert_eq!(bob.name, "Bob");

    // Place ships
    alice
        .grid
        .place_ship(Ship::Carrier, (0, 1), Direction::Horizontal)?;
    bob.grid
        .place_ship(Ship::Carrier, (3, 0), Direction::Vertical)?;

    // Let everyone have 5 turns
    for i in 0..5 {
        for turn in game.round() {
            for opponent in turn.opponents.iter() {
                opponent.grid.fire_at(i, i);
            }
        }
    }

    assert_eq!(
        alice.grid.to_string(),
        ["x....", "OXOOO", "..x..", "...x.", "....x"].join("\n")
    );
    assert_eq!(
        bob.grid.to_string(),
        ["x..O.", ".x.O.", "..xO.", "...X.", "...Ox"].join("\n")
    );

    Ok(())
}
