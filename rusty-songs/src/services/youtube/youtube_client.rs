use reqwest::Client;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct YoutubeClient {
    pub api_key: String,
    pub client: Client,
}

impl YoutubeClient {
    fn new(api_key: String) -> Self {
        YoutubeClient {
            api_key,
            client: Client::new(),
        }
    }

    pub fn get_instance() -> Arc<Mutex<YoutubeClient>> {
        static mut SINGLETON: Option<Arc<Mutex<YoutubeClient>>> = None;

        unsafe {
            SINGLETON
                .get_or_insert_with(|| {
                    let api_key = env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be set");
                    Arc::new(Mutex::new(YoutubeClient::new(api_key)))
                })
                .clone()
        }
    }
}
