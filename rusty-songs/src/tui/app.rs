use crate::models::video::Video;
use crate::services::youtube::youtube_service::YoutubeService;
use crate::tui::ui::builder::LayoutBuilder;
use crate::tui::ui::notification::{Notification, NotificationType};
use crate::tui::ui::playback::Playback;
use crate::tui::ui::playlist::Playlist;
use crate::tui::ui::queue::Queue;
use crate::tui::ui::search_bar::SearchBar;
use crossterm::event::{self, poll, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
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

#[derive(Clone)]
pub struct App {
    search_bar: SearchBar,
    playlist: Playlist,
    queue: Queue,
    playback: Playback,
    selected_pane: Pane,
    youtube_service: YoutubeService,
    search_results: Option<Vec<Video>>,
    selected_video: Option<Video>,
    selected_search_index: usize,
    notification: Option<Notification>,
    notification_timeout: Duration,
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
            selected_video: None,
            selected_search_index: 0,
            notification: None,
            notification_timeout: Duration::from_secs(5),
        }
    }

    pub fn set_notification(&mut self, message: String, notif_type: NotificationType) {
        self.notification = Some(Notification::new(message, notif_type));
    }

    pub fn clear_notification(&mut self) {
        self.notification = None;
    }

    pub fn check_notification_timeout(&mut self) {
        if let Some(notification) = &self.notification {
            if notification.should_clear(self.notification_timeout) {
                self.clear_notification();
            }
        }
    }

    pub async fn run(app: Arc<Mutex<App>>) -> Result<(), io::Error> {
        // Initial setup (lock only once for enabling raw mode)
        {
            let mut app_locked = app.lock().await;
            enable_raw_mode()?;
            let stdout = io::stdout();
            execute!(&stdout, Clear(ClearType::All))?;
        }

        // Initialize terminal backend
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;

        loop {
            {
                // Lock app once for rendering logic only
                let mut app_locked = app.lock().await;
                app_locked.check_notification_timeout();

                terminal.draw(|f| {
                    let size = f.size();
                    LayoutBuilder::new()
                        .frame(size)
                        .selected_pane(&app_locked.selected_pane)
                        .search_bar(app_locked.search_bar.clone())
                        .playlist(app_locked.playlist.clone())
                        .queue(app_locked.queue.clone())
                        .playback(app_locked.playback.clone())
                        .search_results(app_locked.search_results.clone())
                        .selected_search_index(app_locked.selected_search_index)
                        .notification(app_locked.notification.as_ref())
                        .build(f);
                })?;
            }

            // Poll for events with a timeout of 100ms, so we don't block the loop
            if poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    let app_clone = Arc::clone(&app); // Clone app before async task

                    // Handling events in an async block
                    match key.code {
                        KeyCode::Char('0') => {
                            let mut app_locked = app_clone.lock().await;
                            app_locked.selected_pane = Pane::SearchBar;
                        }
                        KeyCode::Char('1') => {
                            let mut app_locked = app_clone.lock().await;
                            app_locked.selected_pane = Pane::Playlist;
                        }
                        KeyCode::Char('2') => {
                            let mut app_locked = app_clone.lock().await;
                            app_locked.selected_pane = Pane::Queue;
                        }
                        KeyCode::Char('3') => {
                            let mut app_locked = app_clone.lock().await;
                            app_locked.selected_pane = Pane::Playback;
                        }
                        KeyCode::Char('q') => {
                            disable_raw_mode().unwrap();
                            std::process::exit(0);
                        }
                        KeyCode::Char(c)
                            if matches!(app.lock().await.selected_pane, Pane::SearchBar) =>
                        {
                            let mut app_locked = app_clone.lock().await;
                            app_locked.search_bar.update(c);
                        }
                        KeyCode::Backspace
                            if matches!(app.lock().await.selected_pane, Pane::SearchBar) =>
                        {
                            let mut app_locked = app_clone.lock().await;
                            app_locked.search_bar.delete();
                        }
                        KeyCode::Enter
                            if matches!(app.lock().await.selected_pane, Pane::SearchBar) =>
                        {
                            let query = app.lock().await.search_bar.input.clone();
                            let app_clone_inner = Arc::clone(&app); // Clone app before async task

                            // Async search task for YouTube videos
                            tokio::spawn(async move {
                                let mut app_locked = app_clone_inner.lock().await;
                                let search_results =
                                    app_locked.youtube_service.search_videos(&query, 10).await;
                                match search_results {
                                    Ok(videos) => {
                                        app_locked.search_results = Some(videos);
                                        app_locked.selected_search_index = 0;
                                        app_locked.selected_pane = Pane::SearchResults;
                                    }
                                    Err(err) => {
                                        eprintln!("Error searching YouTube: {}", err);
                                        app_locked.search_results = None;
                                    }
                                }
                                app_locked.search_bar.clear();
                            });
                        }
                        KeyCode::Char('j')
                            if matches!(app.lock().await.selected_pane, Pane::SearchResults) =>
                        {
                            let mut app_locked = app_clone.lock().await;
                            if let Some(ref results) = app_locked.search_results {
                                app_locked.selected_search_index =
                                    (app_locked.selected_search_index + 1).min(results.len() - 1);
                            }
                        }
                        KeyCode::Char('k')
                            if matches!(app.lock().await.selected_pane, Pane::SearchResults) =>
                        {
                            let mut app_locked = app_clone.lock().await;
                            if app_locked.selected_search_index > 0 {
                                app_locked.selected_search_index -= 1;
                            }
                        }
                        KeyCode::Char('i')
                            if matches!(app.lock().await.selected_pane, Pane::SearchResults) =>
                        {
                            let selected_video = {
                                let app_locked = app_clone.lock().await;
                                app_locked.search_results.as_ref().and_then(|results| {
                                    results.get(app_locked.selected_search_index).cloned()
                                })
                            };

                            if let Some(video) = selected_video {
                                if let Some(video_id) = video.id.video_id {
                                    let app_clone_inner = Arc::clone(&app); // Clone app for async task

                                    // Async task for downloading video and converting to MP3
                                    tokio::spawn(async move {
                                        let mut app_locked = app_clone_inner.lock().await;
                                        match app_locked
                                            .youtube_service
                                            .process_video_to_audio(&video_id) // Pass video_id as &str
                                            .await
                                        {
                                            Ok(_) => {
                                                app_locked.set_notification(
                                                    format!(
                                                        "Successfully processed video to MP3: {}",
                                                        video_id
                                                    ),
                                                    NotificationType::Success,
                                                );
                                            }
                                            Err(e) => {
                                                app_locked.set_notification(
                                                    format!(
                                                        "Failed to process video to MP3: {}",
                                                        e
                                                    ),
                                                    NotificationType::Error,
                                                );
                                            }
                                        }
                                    });
                                } else {
                                    // Handle the case where video_id is None
                                    let mut app_locked = app_clone.lock().await;
                                    app_locked.set_notification(
                                        "Failed to process video: Video ID is missing.".to_string(),
                                        NotificationType::Error,
                                    );
                                }
                            }
                        }
                        KeyCode::Esc
                            if matches!(app.lock().await.selected_pane, Pane::SearchResults) =>
                        {
                            let mut app_locked = app_clone.lock().await;
                            app_locked.selected_pane = Pane::SearchBar;
                            app_locked.search_results = None;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
