use gtk::prelude::*;
use gtk::{ApplicationWindow, Box,  Orientation, Paned, TextView};

use crate::core::NotesMD;
use crate::layout::{Edit, View};

pub fn build_ui(application: &gtk::Application) {
  let window: ApplicationWindow = ApplicationWindow::new(application);
  let mut notesmd = NotesMD::new();

  notesmd.new_sheet();

  let paned: Paned = Paned::new(Orientation::Horizontal);
  let edit: TextView = Edit::get_box(notesmd);
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
