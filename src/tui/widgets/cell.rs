use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::Stylize,
    style::Style,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::game::{Cell, Point};

pub struct CellWidget<'a> {
    point: Point,
    cell: &'a Cell,
    with_ships: bool,
}

impl<'a> CellWidget<'a> {
    pub fn new(point: Point, cell: &'a Cell, with_ships: bool) -> Self {
        Self {
            point,
            cell,
            with_ships,
        }
    }
}

impl Widget for CellWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = match (self.cell.has_ship(), self.cell.is_hit()) {
            (Some(_ship), false) if self.with_ships => "  ".on_white(),
            (Some(_ship), true) if self.with_ships => "  ".on_red(),
            (_, false) => "  ".on_blue(),
            (_, true) => "  ".on_yellow(),
        };
        Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM | Borders::RIGHT)
                    .border_style(Style::new().dark_gray()),
            )
            .render(area, buf);
    }
}
