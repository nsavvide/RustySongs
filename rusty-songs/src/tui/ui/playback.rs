use crate::models::playback::playback_info::PlaybackInfo;
use crate::models::playback::playback_state::PlaybackState;
use crate::tui::ui::color_theme::ColorTheme;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::widgets::{Block, Borders, Gauge, Paragraph};
use tui::Frame;

#[derive(Clone)]
pub struct Playback {
    pub state: PlaybackState,
    pub current_song: Option<String>,
    pub current_time: u64,
    pub total_time: u64,
}

impl Playback {
    pub fn new() -> Self {
        Playback {
            state: PlaybackState::Stopped,
            current_song: None,
            current_time: 0,
            total_time: 0,
        }
    }

    pub fn update_state(&mut self, playback_info: PlaybackInfo) {
        self.state = playback_info.state;
        self.current_song = playback_info.current_song;
        self.current_time = playback_info.current_time;
        self.total_time = playback_info.total_time;
    }

    pub fn render_with_style<B: Backend>(&self, f: &mut Frame<B>, area: Rect, style: Style) {
        let theme = ColorTheme::catppuccin_mocha();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(area);

        // Determine the display message based on the playback state
        let song_info = match self.state {
            PlaybackState::Playing => format!(
                "Playing: {}",
                self.current_song
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string())
            ),
            PlaybackState::Paused => format!(
                "Paused: {}",
                self.current_song
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string())
            ),
            PlaybackState::Stopped => "Stopped".to_string(),
        };

        let paragraph = Paragraph::new(song_info)
            .block(Block::default().borders(Borders::ALL).title("Now Playing"))
            .style(style);

        f.render_widget(paragraph, chunks[0]);

        // Display the progress bar only if a song is playing or paused
        if self.state == PlaybackState::Playing || self.state == PlaybackState::Paused {
            let percentage = if self.total_time > 0 {
                self.current_time as f64 / self.total_time as f64
            } else {
                0.0
            };
            let gauge = Gauge::default()
                .block(Block::default().borders(Borders::ALL))
                .gauge_style(Style::default().fg(theme.accent3))
                .percent((percentage * 100.0) as u16);

            f.render_widget(gauge, chunks[1]);
        } else {
            // Clear the progress bar area when stopped
            let empty_block = Block::default().borders(Borders::ALL);
            f.render_widget(empty_block, chunks[1]);
        }
    }
}
