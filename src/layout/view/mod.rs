use gtk::prelude::*;
use gtk::{Box, Orientation};

mod style;

pub struct View;

impl View {
  pub fn get_box() -> Box {
    let provider = gtk::CssProvider::new();
    // let view: Box = builder.get_object("view").expect("Couldn't get view");
    let view = Box::new(Orientation::Vertical, 0);

    provider.load_from_data(style::VIEW_STYLE.as_bytes())
      .expect("Failed to load VIEW CSS");

    gtk::StyleContext::add_provider_for_screen(
      &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
      &provider,
      gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    view.set_widget_name("view");
    view.set_property_height_request(100);
    view.set_property_width_request(100);

    return view;
  }
}
