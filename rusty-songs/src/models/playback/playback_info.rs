use crate::models::playback::playback_state::PlaybackState;

#[derive(Clone)]
pub struct PlaybackInfo {
    pub state: PlaybackState,
    pub current_song: Option<String>,
    pub current_time: u64,
    pub total_time: u64,
}
