use gtk::prelude::*;
use gtk::{ApplicationWindow, Box,  Orientation, Paned};

use crate::layout::{Edit, View};

pub fn build_ui(application: &gtk::Application) {
  let window: ApplicationWindow = ApplicationWindow::new(application);

  let paned: Paned = Paned::new(Orientation::Horizontal);
  let edit: Box = Edit::get_box();
  let view: Box = View::get_box();

  paned.add1(&edit);
  paned.add2(&view);
  paned.show_all();

  window.set_position(gtk::WindowPosition::Center);
  window.set_application(Some(application));
  window.set_title("NotesMD");
  window.set_resizable(true);
  window.set_default_size(640, 480);
  window.add(&paned);

  window.show();
}
