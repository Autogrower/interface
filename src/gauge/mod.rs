use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct Gauge(ObjectSubclass<imp::Gauge>) @extends gtk::Widget;
}

impl Default for Gauge {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl Gauge {
    pub fn new() -> Self {
        Self::default()
    }
}
