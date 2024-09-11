use std::time::{Duration, Instant};
use tui::style::{Color, Style};

#[derive(Clone, PartialEq)]
pub enum NotificationType {
    Info,
    Success,
    Error,
}

#[derive(Clone)]
pub struct Notification {
    pub message: String,
    pub notif_type: NotificationType,
    pub timestamp: Instant, // When the notification was created
}

impl Notification {
    pub fn new(message: String, notif_type: NotificationType) -> Self {
        Notification {
            message,
            notif_type,
            timestamp: Instant::now(),
        }
    }

    pub fn style(&self) -> Style {
        match self.notif_type {
            NotificationType::Info => Style::default().fg(Color::Blue),
            NotificationType::Success => Style::default().fg(Color::Green),
            NotificationType::Error => Style::default().fg(Color::Red),
        }
    }

    pub fn should_clear(&self, timeout: Duration) -> bool {
        self.timestamp.elapsed() > timeout
    }
}

