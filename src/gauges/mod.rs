use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct Gauges(ObjectSubclass<imp::Gauges>) @extends gtk::Widget;
}
