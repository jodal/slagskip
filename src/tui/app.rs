use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use eyre::Result;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Stylize,
    style::{Modifier, Style},
    symbols::border,
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Paragraph, Widget,
    },
    Frame,
};

use crate::game::{Active, Game, GameResult};

use super::{cursor::Cursor, terminal, widgets::PlayerWidget};

#[derive(Debug)]
pub struct App {
    game: Game<Active>,
    cursor: Cursor,
    message: Option<String>,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        // Only used for test setup.
        Self::new(Game::<Active>::default())
    }
}

impl App {
    pub fn new(game: Game<Active>) -> Self {
        assert_eq!(game.players.len(), 2);
        let grid_size = game.grid_size;
        App {
            game,
            cursor: Cursor::new(grid_size, grid_size),
            message: None,
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
                if self.game.result().is_some() {
                    return; // Game has ended
                }

                if let Some(_fire) = self.game.players[1].fire_at(self.cursor.point) {
                    self.game.players[0].fire_at_random();
                }

                match self.game.result() {
                    Some(GameResult::Winner(player)) => {
                        self.message = Some(format!("{} won!", player.name));
                    }
                    Some(GameResult::Draw) => {
                        self.message = Some("It's a draw!".into());
                    }
                    None => {}
                }
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

        for (i, player) in self.game.players.iter().enumerate() {
            PlayerWidget::new(
                player,
                i == 0,
                if i == 0 {
                    None
                } else {
                    Some(self.cursor.point)
                },
            )
            .render(players_rects[i], buf);
        }

        if let Some(message) = &self.message {
            let message_area = centered_rect(20, 3, area);
            let message_paragraph = Paragraph::new(message.clone())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Thick)
                        .border_style(Style::default().yellow().add_modifier(Modifier::BOLD))
                        .style(Style::default().add_modifier(Modifier::BOLD)),
                )
                .alignment(Alignment::Center)
                .style(Style::default());
            message_paragraph.render(message_area, buf);
        }
    }
}

pub(crate) fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(area.height / 2 - height / 2),
            Constraint::Length(height),
            Constraint::Length(area.height / 2 - height / 2),
        ])
        .split(area);
    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(area.width / 2 - width / 2),
            Constraint::Length(width),
            Constraint::Length(area.width / 2 - width / 2),
        ])
        .split(vertical_layout[1]);
    horizontal_layout[1]
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
