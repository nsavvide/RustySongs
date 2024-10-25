use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Block, Borders, Row, Table};
use tui::Frame;

use crate::models::song::Song;
use crate::tui::ui::color_theme::ColorTheme;
use crate::utils::format::format_duration;

#[derive(Clone)]
pub struct Queue {
    pub songs: Vec<Song>,
}

impl Queue {
    pub fn new(songs: Vec<Song>) -> Self {
        Queue { songs }
    }

    pub fn add_song(&mut self, song: Song) {
        self.songs.push(song);
    }

    pub fn render_with_style<B: Backend>(
        &self,
        f: &mut Frame<B>,
        area: Rect,
        style: Style,
        selected_index: usize,
    ) {
        let theme = ColorTheme::catppuccin_mocha();

        // Create rows for each song
        let rows: Vec<Row> = self
            .songs
            .iter()
            .enumerate()
            .map(|(i, song)| {
                let order = format!("{}", i + 1); // Order starts from 1
                let title = song
                    .title
                    .strip_suffix(".mp3")
                    .unwrap_or(&song.title)
                    .to_string(); // Remove ".mp3" suffix if present
                let duration = format_duration(song.duration); // Format the duration

                Row::new(vec![order, title, duration]).style(if i == selected_index {
                    Style::default().fg(theme.highlight) // Highlight the selected row
                } else {
                    Style::default().fg(theme.text) // Default text color for other rows
                })
            })
            .collect();

        let table = Table::new(rows)
            .block(Block::default().borders(Borders::ALL).title("Queue [2]")) // Set a title and borders
            .style(style) // Apply the passed-in style
            .widths(&[
                tui::layout::Constraint::Percentage(10), // Order
                tui::layout::Constraint::Percentage(60), // Title
                tui::layout::Constraint::Percentage(30), // Duration
            ]);

        f.render_widget(table, area);
    }
}
