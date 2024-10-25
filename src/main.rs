extern crate dotenv;

use dotenv::dotenv;
use rusty_songs::tui::app;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Arc::new(Mutex::new(app::App::new()));

    if let Err(e) = app::App::run(app.clone()).await {
        eprintln!("Application error: {}", e);
    }
}
