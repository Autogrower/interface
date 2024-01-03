use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/example/calendar.ui")]
pub struct Calendar {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Calendar {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "Calendar";
    type Type = super::Calendar;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);

        klass.set_layout_manager_type::<gtk::BoxLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Calendar {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Calendar {}
