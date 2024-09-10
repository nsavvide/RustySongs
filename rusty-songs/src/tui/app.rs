use crate::models::video::Video;
use crate::services::youtube::youtube_service::YoutubeService;
use crate::tui::ui::builder::LayoutBuilder;
use crate::tui::ui::playback::Playback;
use crate::tui::ui::playlist::Playlist;
use crate::tui::ui::queue::Queue;
use crate::tui::ui::search_bar::SearchBar;
use crossterm::event::{self, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

#[derive(Clone, PartialEq)]
pub enum Pane {
    SearchBar,
    Playlist,
    Queue,
    Playback,
    SearchResults,
}

pub struct App {
    search_bar: SearchBar,
    playlist: Playlist,
    queue: Queue,
    playback: Playback,
    selected_pane: Pane,
    youtube_service: YoutubeService,
    search_results: Option<Vec<Video>>,
}

impl App {
    pub fn new() -> Self {
        App {
            search_bar: SearchBar::new(),
            playlist: Playlist::new(vec![
                String::from("classical"),
                String::from("tmp"),
                String::from("yiyang"),
                String::from("misc"),
            ]),
            queue: Queue::new(vec![
                (
                    String::from("4m 52s"),
                    String::from("Artist A"),
                    String::from("Song 1"),
                    String::from("Album X"),
                ),
                (
                    String::from("3m 30s"),
                    String::from("Artist B"),
                    String::from("Song 2"),
                    String::from("Album Y"),
                ),
                (
                    String::from("5m 15s"),
                    String::from("Artist C"),
                    String::from("Song 3"),
                    String::from("Album Z"),
                ),
            ]),
            youtube_service: YoutubeService::new(),
            search_results: None,
            playback: Playback::new("Song 1", 100, 300),
            selected_pane: Pane::SearchBar, // Default to the search bar
        }
    }

    pub async fn run(&mut self) -> Result<(), io::Error> {
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            let selected_pane = &self.selected_pane;

            terminal.draw(|f| {
                let size = f.size();
                LayoutBuilder::new()
                    .frame(size)
                    .selected_pane(selected_pane)
                    .search_bar(self.search_bar.clone())
                    .playlist(self.playlist.clone())
                    .queue(self.queue.clone())
                    .playback(self.playback.clone())
                    .search_results(self.search_results.clone()) // Pass search results
                    .build(f);
            })?;

            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('0') => self.selected_pane = Pane::SearchBar,
                    KeyCode::Char('1') => self.selected_pane = Pane::Playlist,
                    KeyCode::Char('2') => self.selected_pane = Pane::Queue,
                    KeyCode::Char('3') => self.selected_pane = Pane::Playback,
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        break;
                    }

                    // Handle search input and search result pane selection
                    KeyCode::Char(c) if matches!(self.selected_pane, Pane::SearchBar) => {
                        self.search_bar.update(c);
                    }
                    KeyCode::Backspace if matches!(self.selected_pane, Pane::SearchBar) => {
                        self.search_bar.delete();
                    }
                    KeyCode::Enter if matches!(self.selected_pane, Pane::SearchBar) => {
                        let query = self.search_bar.input.clone();
                        let search_results = self.youtube_service.search_videos(&query, 10).await;
                        match search_results {
                            Ok(videos) => {
                                self.search_results = Some(videos);
                                self.selected_pane = Pane::SearchResults; // Switch to search results pane
                            }
                            Err(err) => {
                                eprintln!("Error searching YouTube: {}", err);
                                self.search_results = None;
                            }
                        }
                        self.search_bar.clear();
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

