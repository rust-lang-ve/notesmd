use gtk::prelude::*;
use gtk::{Box, Orientation, ResizeMode};

pub struct Toolbar;

impl Toolbar {
    pub fn get_box() -> Box {
        let header_bar = Box::new(Orientation::Vertical, 0);

        header_bar.set_widget_name("header_bar");
        header_bar.set_size_request(100, 100);
        header_bar.set_property_resize_mode(ResizeMode::Parent);
        header_bar.show();

        return header_bar;
    }
}
