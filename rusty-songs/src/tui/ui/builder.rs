use crate::models::video::Video;
use crate::tui::app::Pane;
use crate::tui::ui::notification::{Notification, NotificationType};
use crate::tui::ui::{playback::Playback, playlist::Playlist, queue::Queue, search_bar::SearchBar};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Frame; // Assuming Notification is already defined

pub struct LayoutBuilder<'a> {
    frame: Option<Rect>,
    search_bar: Option<SearchBar>,
    playlist: Option<Playlist>,
    queue: Option<Queue>,
    playback: Option<Playback>,
    search_results: Option<Vec<Video>>, // Store search results here
    selected_pane: Option<&'a Pane>,
    selected_search_index: Option<usize>,
    notification: Option<&'a Notification>, // Add notification field
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
            notification: None, // Initially no notification
        }
    }

    // Builder method to set the frame (size of the layout)
    pub fn frame(mut self, frame: Rect) -> Self {
        self.frame = Some(frame);
        self
    }

    // Builder method to set the search bar
    pub fn search_bar(mut self, search_bar: SearchBar) -> Self {
        self.search_bar = Some(search_bar);
        self
    }

    // Builder method to set the selected search index
    pub fn selected_search_index(mut self, selected_search_index: usize) -> Self {
        self.selected_search_index = Some(selected_search_index);
        self
    }

    // Builder method to set the playlist
    pub fn playlist(mut self, playlist: Playlist) -> Self {
        self.playlist = Some(playlist);
        self
    }

    // Builder method to set the queue
    pub fn queue(mut self, queue: Queue) -> Self {
        self.queue = Some(queue);
        self
    }

    // Builder method to set the playback section
    pub fn playback(mut self, playback: Playback) -> Self {
        self.playback = Some(playback);
        self
    }

    // Builder method to set the search results
    pub fn search_results(mut self, search_results: Option<Vec<Video>>) -> Self {
        self.search_results = search_results;
        self
    }

    // Builder method to set the selected pane
    pub fn selected_pane(mut self, selected_pane: &'a Pane) -> Self {
        self.selected_pane = Some(selected_pane);
        self
    }

    // Builder method to set the notification
    pub fn notification(mut self, notification: Option<&'a Notification>) -> Self {
        self.notification = notification;
        self
    }

    // Build the layout with the provided components
    pub fn build<B: Backend>(self, f: &mut Frame<B>) {
        // Split the screen into three sections: top, middle, and bottom
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(10), // Top section: Search bar and notification
                    Constraint::Percentage(65), // Middle section: Playlist and Queue
                    Constraint::Percentage(25), // Bottom section: Search results
                ]
                .as_ref(),
            )
            .split(self.frame.unwrap());

        // Render the Search Bar
        if let Some(search_bar) = self.search_bar {
            let style = if matches!(self.selected_pane, Some(Pane::SearchBar)) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            search_bar.render_with_style(f, chunks[0], style);
        }

        // Render the notification in the top-right corner if it exists
        if let Some(notification) = self.notification {
            let style = notification.style(); // Get the appropriate style based on notification type
            let message = &notification.message;

            // Create a Rect to position the notification in the top-right corner
            let notification_chunk = Rect {
                x: chunks[0].x + chunks[0].width - message.len() as u16 - 2, // Right-align the notification
                y: chunks[0].y,
                width: message.len() as u16 + 2,
                height: 1,
            };

            // Render the notification message with borders
            f.render_widget(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Notification")
                    .style(style),
                notification_chunk,
            );
            f.render_widget(
                Paragraph::new(message.as_ref()).style(style),
                notification_chunk,
            );
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

        // Bottom section: Search results
        if let Some(search_results) = &self.search_results {
            let items: Vec<ListItem> = search_results
                .iter()
                .enumerate()
                .map(|(i, video)| {
                    let content =
                        format!("{} - {}", video.snippet.title, video.snippet.channel_title);
                    if Some(i) == self.selected_search_index {
                        ListItem::new(content).style(Style::default().fg(Color::Yellow))
                    } else {
                        ListItem::new(content)
                    }
                })
                .collect();

            let search_result_list = List::new(items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Search Results"),
            );

            f.render_widget(search_result_list, chunks[2]); // Render in the bottom section
        }
    }
}

