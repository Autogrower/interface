use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use crate::gauge::Gauge;

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/example/gauges.ui")]
pub struct Gauges {
    #[template_child]
    ec: TemplateChild<Gauge>,
    #[template_child]
    ph: TemplateChild<Gauge>,
    #[template_child]
    water: TemplateChild<Gauge>,
}

#[glib::object_subclass]
impl ObjectSubclass for Gauges {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "Gauges";
    type Type = super::Gauges;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
        klass.set_css_name("gauges");
        klass.set_layout_manager_type::<gtk::BoxLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Gauges {
    fn constructed(&self) {
        self.ec.set_current(10.0);
        self.ph.set_current(7.0);
        self.water.set_current(90.0);
        // Call "constructed" on parent
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Gauges {}

impl Gauges {}
