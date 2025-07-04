mod app;
mod components;
mod config;
use std::io;
use app::App;

fn main() -> io::Result<()> {
    let mut term = ratatui::init();
    let result = App::default().run(&mut term);
    ratatui::restore();
    result
}
