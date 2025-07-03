mod app;
use std::{io};

use app::App;

fn main() -> io::Result<()> {
    let mut term = ratatui::init();
    let result = App::new().run(&mut term);
    ratatui::restore();
    result
}
