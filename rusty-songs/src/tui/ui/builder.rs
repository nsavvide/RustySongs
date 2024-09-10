use crate::tui::ui::{playback::Playback, playlist::Playlist, queue::Queue, search_bar::SearchBar};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::Frame;

pub struct LayoutBuilder {
    frame: Option<Rect>,
    search_bar: Option<SearchBar>,
    playlist: Option<Playlist>,
    queue: Option<Queue>,
    playback: Option<Playback>,
}

impl LayoutBuilder {
    pub fn new() -> Self {
        LayoutBuilder {
            frame: None,
            search_bar: None,
            playlist: None,
            queue: None,
            playback: None,
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
            search_bar.render(f, chunks[0]);
        }

        // Middle section: Split into playlist and queue
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(chunks[1]);

        if let Some(playlist) = self.playlist {
            playlist.render(f, top_chunks[0]);
        }

        if let Some(queue) = self.queue {
            queue.render(f, top_chunks[1]);
        }

        // Bottom section: Playback progress
        if let Some(playback) = self.playback {
            playback.render(f, chunks[2]);
        }
    }
}

