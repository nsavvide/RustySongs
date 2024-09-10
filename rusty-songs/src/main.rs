extern crate dotenv;

use dotenv::dotenv;
use rusty_songs::tui::app;

fn main() {
    dotenv().ok();

    let mut app = app::App::new();
    app.run();
}
