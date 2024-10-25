use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub items: Vec<Video>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Video {
    pub id: VideoId,
    pub snippet: Snippet,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VideoId {
    #[serde(rename = "videoId")]
    pub video_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Snippet {
    pub title: String,
    pub description: String,
    #[serde(rename = "channelTitle")]
    pub channel_title: String,
}
