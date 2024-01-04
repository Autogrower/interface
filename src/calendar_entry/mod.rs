mod imp;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct CalendarEntry(ObjectSubclass<imp::CalendarEntry>)
        @extends gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for CalendarEntry {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl CalendarEntry {
    pub fn new() -> Self {
        Self::default()
    }
}
