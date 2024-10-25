use crate::models::playback::playback_command::PlaybackCommand;
use crate::models::playback::playback_info::PlaybackInfo;
use crate::models::playback::playback_state::PlaybackState;
use crate::models::song::Song;
use crate::utils::duration::get_mp3_duration;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use tokio::sync::{mpsc, watch};
use tokio::time::{sleep, Duration};

pub struct PlaybackService {
    queue_receiver: watch::Receiver<Vec<Song>>,
    command_sender: mpsc::Sender<PlaybackCommand>,
    command_receiver: mpsc::Receiver<PlaybackCommand>,
    state_sender: watch::Sender<PlaybackInfo>,
    state: PlaybackState,
    current_song_index: usize,
    current_song: Option<String>,
    current_time: u64,
    total_time: u64,
    sink: Option<Sink>,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    music_dir: String,
}

impl PlaybackService {
    pub fn new(
        queue_receiver: watch::Receiver<Vec<Song>>,
        command_sender: mpsc::Sender<PlaybackCommand>,
        command_receiver: mpsc::Receiver<PlaybackCommand>,
        state_sender: watch::Sender<PlaybackInfo>,
        music_dir: String,
    ) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        PlaybackService {
            queue_receiver,
            command_sender,
            command_receiver,
            state_sender,
            state: PlaybackState::Stopped,
            current_song_index: 0,
            current_song: None,
            current_time: 0,
            total_time: 0,
            sink: None,
            _stream,
            stream_handle,
            music_dir,
        }
    }

    pub async fn run(mut self) {
        loop {
            tokio::select! {
                _ = self.queue_receiver.changed() => {
                    let queue = self.queue_receiver.borrow().clone();
                    self.handle_queue_update(queue).await;
                }
                Some(command) = self.command_receiver.recv() => {
                    self.handle_command(command).await;
                }
                _ = sleep(Duration::from_secs(1)), if self.state == PlaybackState::Playing => {
                    self.update_playback_progress().await;
                }
            }
        }
    }

    async fn update_playback_progress(&mut self) {
        self.current_time += 1;
        if self.current_time >= self.total_time {
            self.current_time = self.total_time;
            self.stop_playback();
            // Optionally, automatically play the next song
            self.next_song().await;
        } else {
            let playback_info = PlaybackInfo {
                state: self.state.clone(),
                current_song: self.current_song.clone(),
                current_time: self.current_time,
                total_time: self.total_time,
            };
            let _ = self.state_sender.send(playback_info);
        }
    }

    async fn handle_queue_update(&mut self, queue: Vec<Song>) {
        // If not playing, start playing the first song in the queue
        if self.state == PlaybackState::Stopped {
            if let Some(next_song) = queue.get(self.current_song_index) {
                self.play_song(next_song.clone()).await;
            }
        }
    }

    async fn handle_command(&mut self, command: PlaybackCommand) {
        match command {
            PlaybackCommand::Play => match self.state {
                PlaybackState::Paused => {
                    if let Some(ref sink) = self.sink {
                        sink.play();
                        self.state = PlaybackState::Playing;
                        let playback_info = PlaybackInfo {
                            state: self.state.clone(),
                            current_song: self.current_song.clone(),
                            current_time: self.current_time,
                            total_time: self.total_time,
                        };
                        let _ = self.state_sender.send(playback_info);
                    }
                }
                PlaybackState::Stopped => {
                    let queue = self.queue_receiver.borrow().clone();
                    if let Some(song) = queue.get(self.current_song_index) {
                        self.play_song(song.clone()).await;
                    }
                }
                _ => {}
            },
            PlaybackCommand::Pause => {
                if self.state == PlaybackState::Playing {
                    if let Some(ref sink) = self.sink {
                        sink.pause();
                        self.state = PlaybackState::Paused;
                        let playback_info = PlaybackInfo {
                            state: self.state.clone(),
                            current_song: self.current_song.clone(),
                            current_time: self.current_time,
                            total_time: self.total_time,
                        };
                        let _ = self.state_sender.send(playback_info);
                    }
                }
            }
            PlaybackCommand::Stop => {
                self.stop_playback();
            }
            PlaybackCommand::Next => {
                self.next_song().await;
            }
            PlaybackCommand::Previous => {
                self.previous_song().await;
            }
        }
    }

    async fn play_song(&mut self, song: Song) {
        self.stop_playback();

        let song_title = &song.title;
        let mut song_path = PathBuf::from(&self.music_dir);
        song_path.push(format!("{}.mp3", song_title));

        if !song_path.exists() {
            eprintln!("Song file not found: {:?}", song_path);
            return;
        }

        let sink = Sink::try_new(&self.stream_handle).unwrap();

        let file = match File::open(&song_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to open song file: {:?}", e);
                return;
            }
        };

        let source = match Decoder::new(BufReader::new(file)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to decode song file: {:?}", e);
                return;
            }
        };

        sink.append(source);
        self.sink = Some(sink);
        self.state = PlaybackState::Playing;

        self.current_song = Some(song.title.clone());
        self.current_time = 0;
        self.total_time = get_mp3_duration(&song_path)
            .map(|duration| duration.round() as u64)
            .unwrap_or(0);

        self.state = PlaybackState::Playing;

        let playback_info = PlaybackInfo {
            state: self.state.clone(),
            current_song: self.current_song.clone(),
            current_time: self.current_time,
            total_time: self.total_time,
        };
        let _ = self.state_sender.send(playback_info);

        // Clone necessary variables for the playback completion task
        let sink_clone = self.sink.clone().unwrap();
        let state_sender_clone = self.state_sender.clone();
        let command_sender_clone = self.command_sender.clone();
        let current_song = self.current_song.clone();
        let total_time = self.total_time;

        // Spawn a task to wait until the song finishes playing
        tokio::task::spawn_local(async move {
            sink_clone.sleep_until_end();
            // Notify that the song has finished playing
            let playback_info = PlaybackInfo {
                state: PlaybackState::Stopped,
                current_song: current_song.clone(),
                current_time: total_time,
                total_time,
            };
            let _ = state_sender_clone.send(playback_info);

            // Optionally, send a command to play the next song
            let _ = command_sender_clone.send(PlaybackCommand::Next).await;
        });
    }

    async fn next_song(&mut self) {
        let queue = self.queue_receiver.borrow().clone();
        if self.current_song_index + 1 < queue.len() {
            self.current_song_index += 1;
            if let Some(next_song) = queue.get(self.current_song_index) {
                self.play_song(next_song.clone()).await;
            }
        } else {
            self.stop_playback();
        }
    }

    async fn previous_song(&mut self) {
        if self.current_song_index > 0 {
            self.current_song_index -= 1;
            let queue = self.queue_receiver.borrow().clone();
            if let Some(prev_song) = queue.get(self.current_song_index) {
                self.play_song(prev_song.clone()).await;
            }
        }
    }

    fn stop_playback(&mut self) {
        if let Some(ref sink) = self.sink {
            sink.stop();
        }
        self.sink = None;
        self.state = PlaybackState::Stopped;
        self.current_song = None;
        self.current_time = 0;
        self.total_time = 0;

        let playback_info = PlaybackInfo {
            state: self.state.clone(),
            current_song: self.current_song.clone(),
            current_time: self.current_time,
            total_time: self.total_time,
        };
        let _ = self.state_sender.send(playback_info);
    }
}

