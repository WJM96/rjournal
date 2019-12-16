#[macro_use]
mod util;
mod app;
mod file_editor;

use app::App;
use gio::prelude::*;
use std::env::args;

fn main() {
    util::setup_logger().expect("Couldn't set up logger.");

    let application = gtk::Application::new(Some("com.github.WJM96.rjournal"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|gtk_app| {
        App::run(gtk_app);
    });

    application.run(&args().collect::<Vec<_>>());
}
