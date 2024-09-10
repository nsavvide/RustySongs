use crate::models::video::Video;
use crate::services::youtube::youtube_client::YoutubeClient;
use crate::services::youtube::youtube_request_builder::YoutubeRequestBuilder;
use std::error::Error;
use std::sync::{Arc, Mutex};

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
}
