use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Gauge, Paragraph};
use tui::Frame;

#[derive(Clone)]
pub struct Playback {
    pub current_song: String,
    pub current_time: u64,
    pub total_time: u64,
}

impl Playback {
    pub fn new(song: &str, current_time: u64, total_time: u64) -> Self {
        Playback {
            current_song: song.to_string(),
            current_time,
            total_time,
        }
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        // Display song info
        let song_info = format!(
            "Playing: {} - {}/{}",
            self.current_song, self.current_time, self.total_time
        );
        let paragraph = Paragraph::new(song_info)
            .block(Block::default().borders(Borders::ALL).title("Now Playing"));

        f.render_widget(paragraph, area);

        // Display the progress bar
        let percentage = self.current_time as f64 / self.total_time as f64;
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Yellow))
            .percent((percentage * 100.0) as u16);

        f.render_widget(gauge, area);
    }
}
