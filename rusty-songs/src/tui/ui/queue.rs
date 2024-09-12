use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Block, Borders, Row, Table};
use tui::Frame;

#[derive(Clone)]
pub struct Queue {
    pub songs: Vec<(String, String, String, String)>, // (Duration, Artist, Title, Album)
}

impl Queue {
    pub fn new(songs: Vec<(String, String, String, String)>) -> Self {
        Queue { songs }
    }

    pub fn render_with_style<B: Backend>(&self, f: &mut Frame<B>, area: Rect, style: Style) {
        let rows: Vec<Row> = self
            .songs
            .iter()
            .map(|(duration, artist, title, album)| {
                Row::new(vec![
                    duration.clone(),
                    artist.clone(),
                    title.clone(),
                    album.clone(),
                ])
            })
            .collect();

        let table = Table::new(rows)
            .block(Block::default().borders(Borders::ALL).title("Queue [2]"))
            .style(style)
            .widths(&[
                tui::layout::Constraint::Percentage(10),
                tui::layout::Constraint::Percentage(30),
                tui::layout::Constraint::Percentage(30),
                tui::layout::Constraint::Percentage(30),
            ]);

        f.render_widget(table, area);
    }
}
