use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::Span;
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

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let search = Paragraph::new(Span::raw(format!("Search: {}", self.input)))
            .block(Block::default().borders(Borders::ALL).title("Search"));

        f.render_widget(search, area);
    }
}
