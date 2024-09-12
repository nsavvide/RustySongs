pub fn format_duration(seconds: f64) -> String {
    let minutes = (seconds / 60.0).floor() as u64;
    let seconds = (seconds % 60.0).floor() as u64;
    format!("{:02}:{:02}", minutes, seconds)
}
