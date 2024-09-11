use crate::models::video::Video;
use crate::services::youtube::youtube_client::YoutubeClient;
use crate::services::youtube::youtube_request_builder::YoutubeRequestBuilder;
use crate::utils::logger::log_to_file;
use crate::utils::video_tools::{compress_mp3, download_video_as_mp3};
use std::error::Error;
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

    pub async fn process_video_to_audio(&self, video_id: &str) -> Result<(), Box<dyn Error>> {
        log_to_file("Processing video to audio").await;
        download_video_as_mp3(video_id).await?;

        let input_mp3 = format!("{}.mp3", video_id); // Assuming the MP3 is saved as video_id.mp3
        let output_mp3 = format!("{}_compressed.mp3", video_id); // Output compressed MP3

        compress_mp3(&input_mp3, &output_mp3).await?;

        println!("Successfully downloaded and compressed the MP3.");
        Ok(())
    }
}
