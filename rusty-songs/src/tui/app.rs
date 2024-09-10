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

pub struct App {
    search_bar: SearchBar,
    playlist: Playlist,
    queue: Queue,
    playback: Playback,
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
            terminal.draw(|f| {
                let size = f.size();
                LayoutBuilder::new()
                    .frame(size)
                    .search_bar(self.search_bar.clone())
                    .playlist(self.playlist.clone())
                    .queue(self.queue.clone())
                    .playback(self.playback.clone())
                    .build(f);
            })?;

            // Handle key events to exit or update the search bar
            if let event::Event::Key(key) = event::read()? {
                match key.code {
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

