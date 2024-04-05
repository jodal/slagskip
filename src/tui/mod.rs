mod app;
mod cursor;
mod terminal;
mod widgets;

use eyre::Result;

use crate::game::Game;

use self::app::App;

pub fn main() -> Result<()> {
    let mut game = Game::new(10);
    game.add_player("Player").place_ships_randomly()?;
    game.add_player("Bot").place_ships_randomly()?;
    let game = game.start()?;

    terminal::install_panic_hook();
    let mut terminal = terminal::init()?;
    let app_result = App::new(game).run(&mut terminal);
    terminal::restore()?;
    app_result
}
