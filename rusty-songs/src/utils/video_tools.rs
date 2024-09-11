use crate::utils::logger::log_to_file;
use std::error::Error;
use std::process::Command;

pub async fn compress_mp3(input_mp3: &str, output_mp3: &str) -> Result<(), Box<dyn Error>> {
    println!("Compressing MP3 file: {}", input_mp3);

    // Use ffmpeg to compress the MP3 to a lower bitrate (e.g., 64k for small size)
    let status = Command::new("ffmpeg")
        .args(&[
            "-i", input_mp3, // Input MP3 file
            "-b:a", "64k",      // Set the audio bitrate to 64 kbps
            output_mp3, // Output compressed MP3 file
        ])
        .status()?;

    if !status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "ffmpeg failed to compress MP3",
        )));
    }

    println!("MP3 compressed successfully.");
    Ok(())
}

pub async fn download_video_as_mp3(video_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://www.youtube.com/watch?v={}", video_id);
    println!("Downloading video with ID: {} as MP3", video_id);

    log_to_file("Downloading video as MP3").await;

    // Define the output format to be video_id.mp3
    let output_template = format!("{}.mp3", video_id);

    // Use yt-dlp to download the video in audio format
    let status = Command::new("yt-dlp")
        .args(&[
            "-x", // Extract audio
            "--audio-format",
            "mp3", // Convert to MP3
            "--audio-quality",
            "0",              // Best quality for MP3 (optional, you can use 5 for lower quality)
            "-o",             // Output template
            &output_template, // Ensures the output file is named video_id.mp3
            &url,
        ])
        .status()?;

    if !status.success() {
        log_to_file(&format!(
            "yt-dlp failed to download video with ID: {}",
            video_id
        ))
        .await;
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "yt-dlp failed to download video",
        )));
    }

    println!(
        "Video downloaded and converted to MP3 successfully: {}.mp3",
        video_id
    );
    Ok(())
}
