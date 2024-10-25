use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Compresses an MP3 file using ffmpeg
pub async fn compress_mp3(input_mp3: &str, output_mp3: &str) -> Result<(), Box<dyn Error>> {
    let status = Command::new("ffmpeg")
        .args(&[
            "-i", input_mp3, // Input MP3 file from MUSIC_DIR
            "-b:a", "64k", // Set the audio bitrate to 64 kbps
            output_mp3,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "ffmpeg failed to compress MP3",
        )));
    }

    Ok(())
}

/// Downloads a YouTube video as an MP3 file using yt-dlp
pub async fn download_video_as_mp3(
    video_id: &str,
    video_title: &str,
) -> Result<(), Box<dyn Error>> {
    // Get the MUSIC_DIR from environment or default to "music"
    let music_dir = env::var("MUSIC_DIR").unwrap_or_else(|_| "music".to_string());

    // Ensure the MP3 file is saved in the MUSIC_DIR
    let output_template = PathBuf::from(&music_dir)
        .join(format!("{}.mp3", video_title))
        .to_str()
        .unwrap()
        .to_string();

    let url = format!("https://www.youtube.com/watch?v={}", video_id);

    let status = Command::new("yt-dlp")
        .args(&[
            "-x", // Extract audio
            "--audio-format",
            "mp3", // Convert to MP3
            "--audio-quality",
            "0", // Best quality for MP3
            "-o",
            &output_template, // Output file template in MUSIC_DIR
            &url,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "yt-dlp failed to download video",
        )));
    }

    Ok(())
}
