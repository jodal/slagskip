use crate::game::{ActivePlayer, Point};
use ratatui::widgets::{Block, Widget};
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
            .constraints([Constraint::Percentage(100)])
            .split(area);

        GridWidget::new(&self.player.grid, self.with_ships, self.cursor).render(layout[0], buf);
    }
}
