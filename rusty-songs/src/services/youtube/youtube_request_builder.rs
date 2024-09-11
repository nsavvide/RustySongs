use crate::models::video::{SearchResponse, Video};
use crate::services::youtube::youtube_client::YoutubeClient;
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct YoutubeRequestBuilder {
    query: String,
    max_results: u8,
    api_client: Arc<Mutex<YoutubeClient>>,
}

impl YoutubeRequestBuilder {
    pub fn new(query: String) -> Self {
        YoutubeRequestBuilder {
            query,
            max_results: 10,
            api_client: YoutubeClient::get_instance(),
        }
    }

    pub fn max_results(mut self, max_results: u8) -> Self {
        self.max_results = max_results;
        self
    }

    pub async fn send(self) -> Result<Vec<Video>, Box<dyn Error>> {
        let url = format!(
            "{}?part=snippet&q={}&maxResults={}&type=video&key={}",
            env::var("YOUTUBE_API_URL").expect("YOUTUBE_API_URL must be set"),
            self.query,
            self.max_results,
            self.api_client.lock().await.api_key
        );

        let client = &self.api_client.lock().await.client;
        let response: SearchResponse = client.get(&url).send().await?.json().await?;

        Ok(response.items)
    }
}
