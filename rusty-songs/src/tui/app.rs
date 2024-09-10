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

#[derive(Clone)]
pub enum Pane {
    SearchBar,
    Playlist,
    Queue,
    Playback,
}

pub struct App {
    search_bar: SearchBar,
    playlist: Playlist,
    queue: Queue,
    playback: Playback,
    selected_pane: Pane,
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
            playback: Playback::new("Song 1", 100, 300),
            selected_pane: Pane::SearchBar, // Default to the search bar
        }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        // Enable raw mode for the terminal
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Event loop for rendering and event handling
        loop {
            let selected_pane = &self.selected_pane; // Pass reference of selected pane with non-static lifetime

            terminal.draw(|f| {
                let size = f.size();
                LayoutBuilder::new()
                    .frame(size)
                    .selected_pane(selected_pane) // Pass the reference to selected pane
                    .search_bar(self.search_bar.clone())
                    .playlist(self.playlist.clone())
                    .queue(self.queue.clone())
                    .playback(self.playback.clone())
                    .build(f);
            })?;

            // Handle key events for selecting panes or exiting
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('0') => {
                        self.selected_pane = Pane::SearchBar;
                    }
                    KeyCode::Char('1') => {
                        self.selected_pane = Pane::Playlist;
                    }
                    KeyCode::Char('2') => {
                        self.selected_pane = Pane::Queue;
                    }
                    KeyCode::Char('3') => {
                        self.selected_pane = Pane::Playback;
                    }
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        break;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

