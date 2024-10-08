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
    selected_playlist_song_index: usize,
    notification: Option<Notification>,
    notification_timeout: Duration,
    downloading_video_index: Option<usize>,
    selected_queue_song_index: usize,
}

impl App {
    pub fn new() -> Self {
        App {
            search_bar: SearchBar::new(),
            playlist: Playlist::new(),
            queue: Queue::new(vec![]),
            youtube_service: YoutubeService::new(),
            search_results: None,
            playback: Playback::new("Song 1", 100, 300),
            selected_pane: Pane::SearchBar, // Default to the search bar

            selected_video: None,
            selected_search_index: 0,
            selected_playlist_song_index: 0,
            notification: None,
            notification_timeout: Duration::from_secs(5),
            downloading_video_index: None,
            selected_queue_song_index: 0,
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
            let _app_locked = app.lock().await;
            enable_raw_mode()?;
            let stdout = io::stdout();
            execute!(&stdout, Clear(ClearType::All))?;
        }

        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;

        loop {
            {
                let mut app_locked = app.lock().await;
                app_locked.check_notification_timeout();

                app_locked.playlist.load_playlist();

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
                        .selected_playlist_song_index(app_locked.selected_playlist_song_index)
                        .downloading_video_index(app_locked.downloading_video_index)
                        .notification(app_locked.notification.as_ref())
                        .selected_queue_song_index(app_locked.selected_queue_song_index)
                        .build(f);
                })?;
            }

            // Poll for events with a timeout of 100ms, so we don't block the loop
            if poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    let app_clone = Arc::clone(&app); // Clone app before async task

                    match key.code {
                        KeyCode::Char('0') => {
                            let mut app_locked = app_clone.lock().await;
                            app_locked.selected_pane = Pane::SearchBar;
                            app_locked.search_results = None; // Clear search results when moving away
                        }
                        KeyCode::Char('1') => {
                            let mut app_locked = app_clone.lock().await;
                            app_locked.selected_pane = Pane::Playlist;
                            app_locked.search_results = None; // Clear search results when moving away
                        }
                        KeyCode::Char('2') => {
                            let mut app_locked = app_clone.lock().await;
                            app_locked.selected_pane = Pane::Queue;
                            app_locked.search_results = None; // Clear search results when moving away
                        }
                        KeyCode::Char('3') => {
                            let mut app_locked = app_clone.lock().await;
                            app_locked.selected_pane = Pane::Playback;
                            app_locked.search_results = None; // Clear search results when moving away
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

                                    let selected_index = app.lock().await.selected_search_index; // Get selected index
                                    {
                                        let mut app_locked = app.lock().await;
                                        app_locked.downloading_video_index = Some(selected_index);
                                    }

                                    // Async task for downloading video and converting to MP3
                                    tokio::spawn(async move {
                                        let mut app_locked = app_clone_inner.lock().await;

                                        match app_locked
                                            .youtube_service
                                            .process_video_to_audio(&video_id, &video.snippet.title) // Pass video_id as &str
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

                                        app_locked.downloading_video_index = None;
                                    });
                                } else {
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
                        // Playlist Controls
                        KeyCode::Char('j')
                            if matches!(app.lock().await.selected_pane, Pane::Playlist) =>
                        {
                            let mut app_locked = app_clone.lock().await;
                            let playlist_len = app_locked.playlist.songs.len(); // Assuming playlist has a 'songs' field
                            app_locked.selected_playlist_song_index =
                                (app_locked.selected_playlist_song_index + 1).min(playlist_len - 1);
                        }

                        KeyCode::Char('k')
                            if matches!(app.lock().await.selected_pane, Pane::Playlist) =>
                        {
                            let mut app_locked = app_clone.lock().await;
                            if app_locked.selected_playlist_song_index > 0 {
                                app_locked.selected_playlist_song_index -= 1;
                            }
                        }
                        KeyCode::Char('d')
                            if matches!(app.lock().await.selected_pane, Pane::Playlist) =>
                        {
                            let mut app_locked = app_clone.lock().await;
                            if app_locked.selected_playlist_song_index
                                < app_locked.playlist.songs.len()
                            {
                                let _index = app_locked.selected_playlist_song_index;

                                app_locked.playlist.remove_song(_index);

                                if app_locked.playlist.songs.is_empty() {
                                    app_locked.selected_playlist_song_index = 0;
                                } else if app_locked.selected_playlist_song_index
                                    >= app_locked.playlist.songs.len()
                                {
                                    app_locked.selected_playlist_song_index =
                                        app_locked.playlist.songs.len() - 1;
                                }
                            }
                        }
                        KeyCode::Char('a')
                            if matches!(app.lock().await.selected_pane, Pane::Playlist) =>
                        {
                            let mut app_locked = app_clone.lock().await;
                            if app_locked.selected_playlist_song_index
                                < app_locked.playlist.songs.len()
                            {
                                let _index = app_locked.selected_playlist_song_index;
                                let _song = app_locked.playlist.songs[_index].clone();

                                app_locked.queue.add_song(_song);
                            }
                        }

                        // Queue Controls
                        KeyCode::Char('j')
                            if matches!(app.lock().await.selected_pane, Pane::Queue) =>
                        {
                            let mut app_locked = app_clone.lock().await;
                            let queue_len = app_locked.queue.songs.len();
                            app_locked.selected_queue_song_index =
                                (app_locked.selected_queue_song_index + 1).min(queue_len - 1);
                        }

                        KeyCode::Char('k')
                            if matches!(app.lock().await.selected_pane, Pane::Queue) =>
                        {
                            let mut app_locked = app_clone.lock().await;
                            if app_locked.selected_queue_song_index > 0 {
                                app_locked.selected_queue_song_index -= 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
