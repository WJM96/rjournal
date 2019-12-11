use gio::prelude::*;

use std::env::args;

#[macro_use]
mod util;
mod actions;
mod ui;
use ui::*;

fn main() {
    util::setup_logger().expect("Couldn't set up logger.");

    let application = gtk::Application::new(Some("com.github.WJM96.rjournal"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
