use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use eyre::Result;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    prelude::Stylize,
    symbols::border,
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, Borders, Widget,
    },
    Frame,
};

use crate::game::ActiveGame;

use super::terminal;

#[derive(Debug)]
pub struct App {
    game: ActiveGame,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self::new(ActiveGame::default())
    }
}

impl App {
    pub fn new(game: ActiveGame) -> Self {
        assert_eq!(game.players.len(), 2);
        App { game, exit: false }
    }

    pub fn run(&mut self, terminal: &mut terminal::Type) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            KeyCode::Left => {
                todo!();
            }
            KeyCode::Right => {
                todo!();
            }
            KeyCode::Up => {
                todo!();
            }
            KeyCode::Down => {
                todo!();
            }
            KeyCode::Enter => {
                todo!();
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Slagskip ".bold().yellow());
        let instructions = Title::from(Line::from(vec![
            " Move ".into(),
            "<L,R,U,D>".blue().bold(),
            " Fire ".into(),
            "<Enter>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        block.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quit() -> Result<()> {
        let mut app = App::default();

        app.handle_key_event(KeyCode::Char('q').into());

        assert_eq!(app.exit, true);
        Ok(())
    }
}
