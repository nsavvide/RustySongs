use crate::models::video::Video;
use crate::services::youtube::youtube_client::YoutubeClient;
use crate::services::youtube::youtube_request_builder::YoutubeRequestBuilder;
use crate::utils::video_tools::{compress_mp3, download_video_as_mp3};
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct YoutubeService {
    api_client: Arc<Mutex<YoutubeClient>>,
}

impl YoutubeService {
    pub fn new() -> Self {
        YoutubeService {
            api_client: YoutubeClient::get_instance(),
        }
    }

    pub async fn search_videos(
        &self,
        query: &str,
        max_results: u8,
    ) -> Result<Vec<Video>, Box<dyn Error>> {
        let builder = YoutubeRequestBuilder::new(query.to_string()).max_results(max_results);

        let videos = builder.send().await?;
        Ok(videos)
    }

    pub async fn process_video_to_audio(
        &self,
        video_id: &str,
        video_title: &str,
    ) -> Result<(), Box<dyn Error>> {
        let music_dir = env::var("MUSIC_DIR").unwrap_or_else(|_| "music".to_string());

        // Step 1: Download the video and save as MP3
        download_video_as_mp3(video_id, video_title).await?;

        // Define the file paths within MUSIC_DIR
        let original_file_name = format!("{}.mp3", video_title);
        let temp_file_name = format!("{}.temp.mp3", video_title); // Temporary file for compression

        let original_path = PathBuf::from(&music_dir).join(&original_file_name);
        let temp_path = PathBuf::from(&music_dir).join(&temp_file_name);

        // Step 2: Compress the MP3 to a temporary file
        compress_mp3(original_path.to_str().unwrap(), temp_path.to_str().unwrap()).await?;

        // Step 3: Delete the original file
        if let Err(e) = fs::remove_file(&original_path) {
            eprintln!("Failed to remove original file: {}", e);
            return Err(Box::new(e));
        }

        // Step 4: Rename the temporary file to the original name
        if let Err(e) = fs::rename(&temp_path, &original_path) {
            eprintln!("Failed to rename temp file to original: {}", e);
            return Err(Box::new(e));
        }

        Ok(())
    }
}
