extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;

use gtk::Application;

mod core;
mod layout;

fn main() {
  let application = Application::new(
    Some("com.github.rust-lang-ve.notes-md"),
    Default::default(),
  ).expect("Failed to initialize NotesMD");

  application.connect_activate(move |app| {
    layout::build_ui(app);
  });

  application.run(&[]);
}
