mod imp;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct Calendar(ObjectSubclass<imp::Calendar>)
        @extends gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for Calendar {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl Calendar {
    pub fn new() -> Self {
        Self::default()
    }
}
