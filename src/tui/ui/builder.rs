use crate::models::video::Video;
use crate::tui::app::Pane;
use crate::tui::ui::color_theme::ColorTheme;
use crate::tui::ui::notification::Notification;
use crate::tui::ui::{playback::Playback, playlist::Playlist, queue::Queue, search_bar::SearchBar};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Frame;

pub struct LayoutBuilder<'a> {
    frame: Option<Rect>,
    search_bar: Option<SearchBar>,
    playlist: Option<Playlist>,
    queue: Option<Queue>,
    playback: Option<Playback>,
    search_results: Option<Vec<Video>>,
    selected_pane: Option<&'a Pane>,
    selected_search_index: Option<usize>,
    selected_playlist_song_index: usize,
    notification: Option<&'a Notification>,
    downloading_video_index: Option<usize>,
    selected_queue_song_index: usize,
    theme: ColorTheme,
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
            selected_search_index: None,
            selected_playlist_song_index: 0,
            downloading_video_index: None,
            selected_queue_song_index: 0,
            notification: None,
            theme: ColorTheme::catppuccin_mocha(),
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

    pub fn selected_search_index(mut self, selected_search_index: usize) -> Self {
        self.selected_search_index = Some(selected_search_index);
        self
    }

    pub fn selected_playlist_song_index(mut self, selected_playlist_song_index: usize) -> Self {
        self.selected_playlist_song_index = selected_playlist_song_index;
        self
    }

    pub fn selected_queue_song_index(mut self, selected_queue_song_index: usize) -> Self {
        self.selected_queue_song_index = selected_queue_song_index;
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

    pub fn downloading_video_index(mut self, index: Option<usize>) -> Self {
        self.downloading_video_index = index;
        self
    }

    pub fn selected_pane(mut self, selected_pane: &'a Pane) -> Self {
        self.selected_pane = Some(selected_pane);
        self
    }

    pub fn notification(mut self, notification: Option<&'a Notification>) -> Self {
        self.notification = notification;
        self
    }

    pub fn build<B: Backend>(self, f: &mut Frame<B>) {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()) // Left and right halves
            .split(self.frame.unwrap());

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref()) // Top: Search Bar, Bottom: Playlist
            .split(main_chunks[0]);

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref()) // Top: Queue, Bottom: Playback
            .split(main_chunks[1]);

        if let Some(search_bar) = self.search_bar {
            let style = if matches!(self.selected_pane, Some(Pane::SearchBar)) {
                Style::default().fg(self.theme.accent2)
            } else {
                Style::default().fg(self.theme.text)
            };
            search_bar.render_with_style(f, left_chunks[0], style);
        }

        if let Some(playlist) = self.playlist.as_ref() {
            let style = if matches!(self.selected_pane, Some(Pane::Playlist)) {
                Style::default().fg(self.theme.accent1)
            } else {
                Style::default().fg(self.theme.text)
            };
            playlist.render_with_style(f, left_chunks[1], style, self.selected_playlist_song_index);
        }

        if let Some(queue) = self.queue {
            let style = if matches!(self.selected_pane, Some(Pane::Queue)) {
                Style::default().fg(self.theme.accent2)
            } else {
                Style::default().fg(self.theme.text)
            };
            queue.render_with_style(f, right_chunks[0], style, self.selected_queue_song_index);
        }

        if let Some(playback) = self.playback {
            let style = if matches!(self.selected_pane, Some(Pane::Playback)) {
                Style::default().fg(self.theme.accent2) // Playback has its own color
            } else {
                Style::default().fg(self.theme.text)
            };
            playback.render_with_style(f, right_chunks[1], style);
        }

        if let Some(search_results) = &self.search_results {
            let search_overlay = Rect {
                x: 2,                                   // Padding from the left edge
                y: 5,                                   // Bring it down a bit (set y to 5)
                width: self.frame.unwrap().width - 4,   // Slightly smaller than full width
                height: self.frame.unwrap().height / 4, // Take 1/4 of the screen height
            };

            // Borrow `self.playlist` once
            let playlist_ref = self.playlist.as_ref();

            let items: Vec<ListItem> = search_results
                .iter()
                .enumerate()
                .map(|(i, video)| {
                    // Check if the video is already downloaded by referencing the borrowed playlist
                    let already_downloaded = if let Some(playlist) = playlist_ref {
                        playlist.songs.iter().any(|song| {
                            song.title.ends_with(".mp3")
                                && song.title == format!("{}.mp3", video.snippet.title)
                        })
                    } else {
                        false // If playlist is None, assume the video hasn't been downloaded
                    };

                    // Determine which symbol to show
                    let download_status_symbol = if already_downloaded {
                        "✅" // Green checkmark for already downloaded
                    } else if Some(i) == self.downloading_video_index {
                        "⏳" // Spinning wheel for currently downloading
                    } else {
                        "⚠️" // Yellow warning symbol for not downloaded yet
                    };

                    // Format the content for each search result item
                    let content = format!(
                        "{} - {} {}",
                        video.snippet.title, video.snippet.channel_title, download_status_symbol
                    );

                    // Highlight the selected search result
                    if Some(i) == self.selected_search_index {
                        ListItem::new(content).style(Style::default().fg(self.theme.accent2))
                    } else {
                        ListItem::new(content).style(Style::default().fg(self.theme.text))
                    }
                })
                .collect();

            let search_result_list = List::new(items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Search Results")
                    .style(
                        Style::default()
                            .bg(self.theme.background)
                            .fg(self.theme.text),
                    )
                    .border_style(Style::default().fg(self.theme.accent1)),
            );

            f.render_widget(search_result_list, search_overlay);
        }
    }
}
