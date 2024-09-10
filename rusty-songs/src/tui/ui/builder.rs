use crate::models::video::Video;
use crate::tui::app::Pane;
use crate::tui::ui::{playback::Playback, playlist::Playlist, queue::Queue, search_bar::SearchBar};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Frame;

pub struct LayoutBuilder<'a> {
    frame: Option<Rect>,
    search_bar: Option<SearchBar>,
    playlist: Option<Playlist>,
    queue: Option<Queue>,
    playback: Option<Playback>,
    search_results: Option<Vec<Video>>, // Store search results here
    selected_pane: Option<&'a Pane>,
}

impl<'a> LayoutBuilder<'a> {
    pub fn new() -> Self {
        LayoutBuilder {
            frame: None,
            search_bar: None,
            playlist: None,
            queue: None,
            playback: None,
            search_results: None,
            selected_pane: None,
        }
    }

    pub fn frame(mut self, frame: Rect) -> Self {
        self.frame = Some(frame);
        self
    }

    pub fn search_bar(mut self, search_bar: SearchBar) -> Self {
        self.search_bar = Some(search_bar);
        self
    }

    pub fn playlist(mut self, playlist: Playlist) -> Self {
        self.playlist = Some(playlist);
        self
    }

    pub fn queue(mut self, queue: Queue) -> Self {
        self.queue = Some(queue);
        self
    }

    pub fn playback(mut self, playback: Playback) -> Self {
        self.playback = Some(playback);
        self
    }

    pub fn search_results(mut self, search_results: Option<Vec<Video>>) -> Self {
        self.search_results = search_results;
        self
    }

    pub fn selected_pane(mut self, selected_pane: &'a Pane) -> Self {
        self.selected_pane = Some(selected_pane);
        self
    }

    pub fn build<B: Backend>(self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(70),
                    Constraint::Percentage(20),
                ]
                .as_ref(),
            )
            .split(self.frame.unwrap());

        // Top section: Search bar
        if let Some(search_bar) = self.search_bar {
            let style = if matches!(self.selected_pane, Some(Pane::SearchBar)) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            search_bar.render_with_style(f, chunks[0], style);
        }

        // Middle section: Split into playlist and queue
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(chunks[1]);

        if let Some(playlist) = self.playlist {
            let style = if matches!(self.selected_pane, Some(Pane::Playlist)) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            playlist.render_with_style(f, top_chunks[0], style);
        }

        if let Some(queue) = self.queue {
            let style = if matches!(self.selected_pane, Some(Pane::Queue)) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            queue.render_with_style(f, top_chunks[1], style);
        }

        // Bottom section: Playback progress
        if let Some(playback) = self.playback {
            let style = if matches!(self.selected_pane, Some(Pane::Playback)) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            playback.render_with_style(f, chunks[2], style);
        }

        // Display search results in an overlapping pane if available
        if let Some(search_results) = &self.search_results {
            let items: Vec<ListItem> = search_results
                .iter()
                .map(|video| {
                    ListItem::new(format!(
                        "{} - {}",
                        video.snippet.title, video.snippet.channel_title
                    ))
                })
                .collect();

            let search_result_list = List::new(items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Search Results"),
            );

            let area = f.size(); // Render on top of the existing UI
            f.render_widget(search_result_list, area);
        }
    }
}

