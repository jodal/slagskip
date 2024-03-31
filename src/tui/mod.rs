mod app;
mod cursor;
mod terminal;
mod widgets;

use eyre::Result;

use crate::game::NewGame;

use self::app::App;

pub fn main() -> Result<()> {
    let mut new_game = NewGame::new(10);
    new_game.add_player("Player").place_ships_randomly()?;
    new_game.add_player("Bot").place_ships_randomly()?;
    let game = new_game.start()?;

    terminal::install_panic_hook();
    let mut terminal = terminal::init()?;
    let app_result = App::new(game).run(&mut terminal);
    terminal::restore()?;
    app_result
}
