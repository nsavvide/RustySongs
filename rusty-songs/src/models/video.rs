use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub items: Vec<Video>,
}

#[derive(Debug, Deserialize)]
pub struct Video {
    pub id: VideoId,
    pub snippet: Snippet,
}

#[derive(Debug, Deserialize)]
pub struct VideoId {
    pub video_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Snippet {
    pub title: String,
    pub description: String,
}
