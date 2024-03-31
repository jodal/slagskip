mod app;
mod terminal;

use eyre::Result;

use crate::game::NewGame;

use self::app::App;

pub fn main() -> Result<()> {
    let mut new_game = NewGame::new(10);
    new_game.add_player("Human").place_ships_randomly()?;
    new_game.add_player("Bot").place_ships_randomly()?;
    let game = new_game.start()?;

    let mut terminal = terminal::init()?;
    let app_result = App::new(game).run(&mut terminal);
    terminal::restore()?;
    app_result
}
