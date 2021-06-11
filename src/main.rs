extern crate gtk;
extern crate gio;

use gio::prelude::*;
use std::env;

mod ui;
mod handler;

fn main() {
    let uiapp = gtk::Application::new(Some("com.github.lerest"), gio::ApplicationFlags::FLAGS_NONE)
                                .expect("Application::new failed");
    uiapp.connect_activate(ui::window::main_window::build);
    uiapp.run(&env::args().collect::<Vec<_>>());
}
