use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/example/calendar-entry.ui")]
pub struct CalendarEntry {
    #[template_child]
    event_name: TemplateChild<gtk::Label>,
    #[template_child]
    event_date: TemplateChild<gtk::Label>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CalendarEntry {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "CalendarEntry";
    type Type = super::CalendarEntry;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
        klass.set_css_name("calendar-entry");
        klass.set_layout_manager_type::<gtk::BoxLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for CalendarEntry {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for CalendarEntry {}

impl CalendarEntry {
    pub fn set_event(&self, event: &str) {
        self.event_name.set_label(event);
    }

    pub fn set_date(&self, date: &str) {
        self.event_date.set_label(date);
    }
}
