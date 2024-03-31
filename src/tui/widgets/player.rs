use crate::game::{ActivePlayer, Point};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Gauge, Widget};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Stylize,
    widgets::{block::Title, Borders},
};

use super::GridWidget;

pub struct PlayerWidget<'a> {
    player: &'a ActivePlayer,
    with_ships: bool,
    cursor: Option<Point>,
}

impl<'a> PlayerWidget<'a> {
    pub fn new(player: &'a ActivePlayer, with_ships: bool, cursor: Option<Point>) -> Self {
        Self {
            player,
            with_ships,
            cursor,
        }
    }
}

impl Widget for PlayerWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let frame_block = Block::default()
            .title(Title::from(self.player.name.clone().bold()).alignment(Alignment::Center))
            .borders(Borders::ALL);
        frame_block.render(area, buf);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(2)
            .vertical_margin(2)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Percentage(100),
            ])
            .split(area);

        let grid_widget = GridWidget::new(&self.player.grid, self.with_ships, self.cursor);
        let grid_width = grid_widget.box_width() as u16;

        let grid_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((layout[2].width - grid_width) / 2),
                Constraint::Length(grid_width),
                Constraint::Min(0),
            ])
            .split(layout[2]);

        Gauge::default()
            .gauge_style(Style::default().fg(Color::Green).bg(Color::Red))
            .ratio(self.player.num_ships_alive() as f64 / self.player.num_ships_total() as f64)
            .label(format!(
                "{}/{}",
                self.player.num_ships_alive(),
                self.player.num_ships_total()
            ))
            .render(layout[0], buf);

        grid_widget.render(grid_layout[1], buf);
    }
}
