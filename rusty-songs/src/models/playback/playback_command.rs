#[derive(Debug, Clone, Copy)]
pub enum PlaybackCommand {
    Play,
    Pause,
    Stop,
    Next,
    Previous,
}
