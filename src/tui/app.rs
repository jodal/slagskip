use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use eyre::Result;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
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

use super::{cursor::Cursor, terminal, widgets::GridWidget};

#[derive(Debug)]
pub struct App {
    game: ActiveGame,
    cursor: Cursor,
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
        let grid_size = game.players[0].grid.size;
        App {
            game,
            cursor: Cursor::new(grid_size, grid_size),
            exit: false,
        }
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
            KeyCode::Up | KeyCode::Char('w') => {
                self.cursor.up();
            }
            KeyCode::Left | KeyCode::Char('a') => {
                self.cursor.left();
            }
            KeyCode::Down | KeyCode::Char('s') => {
                self.cursor.down();
            }
            KeyCode::Right | KeyCode::Char('d') => {
                self.cursor.right();
            }
            KeyCode::Char(' ') => {
                self.game.players[1].fire_at(self.cursor.point);
                // TODO Only give the opponent a turn if we hit a cell that has
                // not been hit before.
                self.game.players[0].fire_at_random();
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
            "WASD or arrows".blue().bold(),
            " Fire ".into(),
            "<Space>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let frame_block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);
        frame_block.render(area, buf);

        let players_rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .vertical_margin(1)
            .horizontal_margin(1)
            .split(area);

        let you_block = Block::default()
            .title(Title::from("Player".bold().blue()).alignment(Alignment::Center))
            .borders(Borders::ALL);
        you_block.render(players_rects[0], buf);
        let you_rects = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(2)
            .vertical_margin(2)
            .constraints([Constraint::Percentage(100)])
            .split(players_rects[0]);
        let you_grid = GridWidget::new(&self.game.players[0].grid, true, None);
        you_grid.render(you_rects[0], buf);

        let opponent_block = Block::default()
            .title(Title::from("Opponent".bold().red()).alignment(Alignment::Center))
            .borders(Borders::ALL);
        opponent_block.render(players_rects[1], buf);
        let opponent_rects = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(2)
            .vertical_margin(2)
            .constraints([Constraint::Percentage(100)])
            .split(players_rects[1]);
        let opponent_grid =
            GridWidget::new(&self.game.players[1].grid, false, Some(self.cursor.point));
        opponent_grid.render(opponent_rects[0], buf);
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
