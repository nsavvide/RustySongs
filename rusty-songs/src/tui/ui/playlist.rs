use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Frame;

#[derive(Clone)]
pub struct Playlist {
    pub items: Vec<String>,
}

impl Playlist {
    pub fn new(items: Vec<String>) -> Self {
        Playlist { items }
    }

    pub fn render_with_style<B: Backend>(&self, f: &mut Frame<B>, area: Rect, style: Style) {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|i| ListItem::new(i.as_str()))
            .collect();

        let playlist = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Playlist"))
            .style(style);

        f.render_widget(playlist, area);
    }
}

