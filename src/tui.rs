use std::io;
use termion::raw::IntoRawMode;
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Row, Table},
    Frame, Terminal,
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.draw(draw)?;

    Ok(())
}

pub fn draw<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let my_block = Block::default().title("My ships").borders(Borders::ALL);
    let my_table = Table::new(vec![
        Row::new(vec!["Hello", "world"]),
        Row::new(vec!["Row11", "Row12", "Row13"]),
    ])
    .column_spacing(1)
    .widths(&[Constraint::Min(1), Constraint::Min(1), Constraint::Min(1)])
    .block(my_block);
    f.render_widget(my_table, chunks[0]);

    let enemy_block = Block::default().title("Enemy ships").borders(Borders::ALL);
    f.render_widget(enemy_block, chunks[1]);
}
