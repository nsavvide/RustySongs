use crate::tui::ui::color_theme::ColorTheme;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Style;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

#[derive(Clone)]
pub struct SearchBar {
    pub input: String,
}

impl SearchBar {
    pub fn new() -> Self {
        SearchBar {
            input: String::new(),
        }
    }

    pub fn update(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn render_with_style<B: Backend>(&self, f: &mut Frame<B>, area: Rect, style: Style) {
        let theme = ColorTheme::catppuccin_mocha();
        let song_label = Span::styled("Song: ", Style::default().fg(theme.accent1)); // Use accent1 for label
        let song_input = Span::styled(&self.input, Style::default().fg(theme.highlight)); // Highlight input in another color

        // Create the paragraph with multiple spans
        let search = Paragraph::new(Spans::from(vec![song_label, song_input]))
            .block(Block::default().borders(Borders::ALL).title("Search [0]"))
            .style(style); // Use block style for the whole block

        f.render_widget(search, area);
    }

    pub fn delete(&mut self) {
        self.input.pop();
    }

    pub fn clear(&mut self) {
        self.input.clear();
    }
}
