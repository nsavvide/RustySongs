use crate::models::song::Song;
use crate::tui::ui::color_theme::ColorTheme;
use crate::utils::duration::get_mp3_duration;
use crate::utils::format::format_duration;
use std::env;
use std::fs;
use std::path::PathBuf;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Style;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Frame;

#[derive(Clone)]
pub struct Playlist {
    pub songs: Vec<Song>,
}

impl Playlist {
    pub fn new() -> Self {
        Playlist { songs: Vec::new() }
    }

    pub fn load_playlist(&mut self) {
        let music_dir = env::var("MUSIC_DIR").unwrap_or_else(|_| "music".to_string());

        for entry in fs::read_dir(&music_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "mp3") {
                let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                if self.songs.iter().any(|song| song.title == file_name) {
                    continue;
                }

                if let Some(duration) = get_mp3_duration(&path) {
                    self.songs.push(Song {
                        title: file_name,
                        duration,
                    });
                }
            }
        }
    }

    pub fn render_with_style<B: Backend>(
        &self,
        f: &mut Frame<B>,
        area: Rect,
        style: Style,
        selected_index: usize,
    ) {
        let theme = ColorTheme::catppuccin_mocha();

        let items: Vec<ListItem> = self
            .songs
            .iter()
            .enumerate() // Enumerate to track the index of each song
            .map(|(i, song)| {
                let duration = format_duration(song.duration);
                let spans = Spans::from(vec![
                    Span::raw(
                        song.title
                            .strip_suffix(".mp3")
                            .unwrap_or(&song.title)
                            .to_string(),
                    ),
                    Span::styled(
                        format!(" [{}]", duration),
                        Style::default().fg(theme.accent2),
                    ),
                ]);

                if i == selected_index {
                    ListItem::new(spans).style(Style::default().fg(theme.highlight))
                } else {
                    ListItem::new(spans).style(Style::default().fg(theme.text))
                }
            })
            .collect();

        let playlist = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Playlist [1]"))
            .style(style);

        f.render_widget(playlist, area);
    }

    pub fn remove_song(&mut self, index: usize) {
        if index < self.songs.len() {
            let song_title = self.songs[index].title.clone();

            self.songs.remove(index);

            let music_dir = std::env::var("MUSIC_DIR").unwrap_or_else(|_| "music".to_string());
            let file_path = PathBuf::from(music_dir).join(song_title);

            if let Err(e) = fs::remove_file(&file_path) {
                eprintln!("Failed to delete file {}: {}", file_path.display(), e);
            }
        }
    }
}
