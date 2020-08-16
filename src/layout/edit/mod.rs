use gdk;
use gtk::prelude::*;
use gtk::{TextView};

mod style;

pub struct Edit;

impl Edit {
  pub fn get_box() -> TextView {
    let provider = gtk::CssProvider::new();
    let edit: TextView = TextView::new();

    if let Some(buff) = edit.get_buffer() {
      buff.connect_insert_text(|buff, text_iter, text| {
        let start = edit.get_iter_at_position(0, 0).unwrap().0;
        let full_text = buff.get_text(&start, text_iter, true).unwrap();

        println!("{} {}", text,  full_text.to_string());
      });
    }

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
