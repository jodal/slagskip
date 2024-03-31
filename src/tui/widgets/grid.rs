use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use crate::game::{Grid, Point};

use super::CellWidget;

pub struct GridWidget<'a> {
    grid: &'a Grid,
    with_ships: bool,
}

impl<'a> GridWidget<'a> {
    pub fn new(grid: &'a Grid, with_ships: bool) -> Self {
        Self { grid, with_ships }
    }
}

impl Widget for GridWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let row_constraints = std::iter::repeat(Constraint::Length(2))
            .take(self.grid.size)
            .collect::<Vec<_>>();
        let col_constraints = std::iter::repeat(Constraint::Length(3))
            .take(self.grid.size)
            .collect::<Vec<_>>();

        let row_rects = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(0)
            .vertical_margin(0)
            .constraints(row_constraints)
            .split(area);
        for (y, row_rect) in row_rects.iter().enumerate() {
            let col_rects = Layout::default()
                .direction(Direction::Horizontal)
                .horizontal_margin(0)
                .vertical_margin(0)
                .constraints(col_constraints.clone())
                .split(*row_rect);
            for (x, cell_rect) in col_rects.iter().enumerate() {
                let point = Point(x, y);
                let cell = self.grid.at(point).unwrap();
                CellWidget::new(point, cell, self.with_ships).render(*cell_rect, buf);
            }
        }
    }
}
