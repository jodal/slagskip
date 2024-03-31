use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::Stylize,
    style::Style,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::game::{Cell, Point};

pub struct CellWidget<'a> {
    cell: &'a Cell,
    with_ships: bool,
    is_active: bool,
}

impl<'a> CellWidget<'a> {
    pub fn new(point: Point, cell: &'a Cell, with_ships: bool, cursor: Option<Point>) -> Self {
        Self {
            cell,
            with_ships,
            is_active: cursor.is_some_and(|c| c == point),
        }
    }

    pub fn content_height() -> usize {
        1 // Line of text
    }

    pub fn content_width() -> usize {
        2 // Line of text
    }

    pub fn box_height() -> usize {
        CellWidget::content_height() + 1 // Border bottom
    }

    pub fn box_width() -> usize {
        CellWidget::content_width() + 1 // Border right
    }
}

impl Widget for CellWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = match (self.cell.has_ship(), self.cell.is_hit()) {
            (Some(_ship), false) if self.with_ships => "  ".on_green(),
            (Some(_ship), true) => "  ".on_red(),
            (_, false) => "  ".on_blue(),
            (_, true) => "  ".on_black(),
        };
        let border_style = if self.is_active {
            Style::new().white()
        } else {
            Style::new().dark_gray()
        };
        Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::RIGHT | Borders::BOTTOM)
                    .border_style(border_style),
            )
            .render(area, buf);
    }
}
