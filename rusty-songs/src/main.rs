extern crate dotenv;

use dotenv::dotenv;
use rusty_songs::tui::app;

#[tokio::main] // Use Tokio runtime to handle async execution
async fn main() {
    dotenv().ok();

    let mut app = app::App::new();
    let _ = app.run().await; // Await the async run method
}
