use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use std::cell::Cell;
use std::cell::RefCell;
#[derive(Debug, Default, glib::Properties, CompositeTemplate)]
#[properties(wrapper_type = super::Gauge)]
#[template(resource = "/org/gtk_rs/example/gauge.ui")]
pub struct Gauge {
    #[property(get, set)]
    max: Cell<f32>,
    #[property(get, set)]
    min: Cell<f32>,
    #[property(get, set)]
    current: Cell<f32>,
    #[property(get, set)]
    name: RefCell<String>,
    #[template_child]
    area: TemplateChild<gtk::DrawingArea>,
}

#[glib::object_subclass]
impl ObjectSubclass for Gauge {
    const NAME: &'static str = "Gauge";
    type Type = super::Gauge;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
        klass.set_layout_manager_type::<gtk::BoxLayout>();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Gauge {
    fn properties() -> &'static [glib::ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        Self::derived_set_property(self, id, value, pspec);
        self.obj().queue_draw();
    }

    fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        Self::derived_property(self, id, pspec)
    }

    fn constructed(&self) {
        self.area.set_draw_func(move |area, ct, width, height| {
            let parent = area.parent().unwrap();

            let cur: f32 = parent.property_value("current").get().unwrap_or(0.0);
            let min: f32 = parent.property_value("min").get().unwrap_or(0.0);
            let max: f32 = parent.property_value("max").get().unwrap_or(0.0);

            let percentage: f32 = cur / max - min;
            let upper_bound: i32 = height - (height as f32 * percentage) as i32;

            let rectangle = gtk::gdk::Rectangle::new(0, upper_bound, width, height);

            ct.add_rectangle(&rectangle);

            ct.set_source_rgb(1., 0., 0.);

            ct.fill().unwrap_or_default();
        });
        // Call "constructed" on parent
        self.parent_constructed();
    }
}

impl WidgetImpl for Gauge {}
