use gdk;
use gtk::prelude::*;
use gtk::{Box, Orientation, TextView};

mod style;

pub struct Edit;

impl Edit {
  pub fn get_box() -> TextView {
    let provider = gtk::CssProvider::new();
    // let edit: Box = builder.get_object("edit").expect("Couldn't get edit");
    let edit: TextView = TextView::new();

    provider.load_from_data(style::EDIT_STYLE.as_bytes())
      .expect("Failed to load EDIT CSS");

    gtk::StyleContext::add_provider_for_screen(
      &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
      &provider,
      gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    edit.set_widget_name("edit");
    edit.set_property_height_request(100);
    edit.set_property_width_request(100);

    return edit;
  }
}
