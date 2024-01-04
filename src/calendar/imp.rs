use gtk::prelude::BoxExt;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use crate::calendar_entry::CalendarEntry;

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/example/calendar.ui")]
pub struct Calendar {
    #[template_child]
    pub events: TemplateChild<gtk::Box>,
}

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
        self.populate();

        // Call "constructed" on parent
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Calendar {}

impl Calendar {
    // actual event parsing is unknown``
    fn populate(&self) {
        let event = CalendarEntry::new();
        event.imp().set_event("Refill X");
        event.imp().set_date("1 Day Ago / 19 December");

        self.add_event(event);

        let event = CalendarEntry::new();
        event.imp().set_event("Refill Y");
        event.imp().set_date("in 2 Weeks/ 3 January");

        self.add_event(event);

        let event = CalendarEntry::new();
        event.imp().set_event("Refill Z");
        event.imp().set_date("in 1 Month / 21 January");

        self.add_event(event);

        let event = CalendarEntry::new();
        event.imp().set_event("Harvest");
        event.imp().set_date("In 2 Months / 21 February");

        self.add_event(event);
    }

    fn add_event(&self, event: CalendarEntry) {
        self.events
            .append(&gtk::Separator::new(gtk::Orientation::Horizontal));
        self.events.append(&event);
    }
}
