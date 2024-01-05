use gtk::cairo::Context;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use std::cell::Cell;
use std::cell::RefCell;
use std::f64::consts::PI;
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
    #[template_child]
    label: TemplateChild<gtk::Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for Gauge {
    const NAME: &'static str = "Gauge";
    type Type = super::Gauge;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
        klass.set_css_name("gauge");
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
        self.obj()
            .bind_property("name", &self.label.get(), "label")
            .sync_create()
            .build();

        self.area.set_draw_func(draw_gauge);
        // Call "constructed" on parent
        self.parent_constructed();
    }
}

impl WidgetImpl for Gauge {}

fn draw_background(_area: &gtk::DrawingArea, ct: &Context, width: i32, height: i32) {
    let radius = (width as f64 + height as f64) / 100.0 / 2.0;

    rounded_rect(ct, 0.0, 0.0, width as f64, height as f64, radius);

    ct.set_source_rgb(217. / 255., 217. / 255., 217. / 255.);

    ct.fill().unwrap_or_default();
}

fn rounded_rect(ct: &Context, x: f64, y: f64, w: f64, h: f64, r: f64) {
    ct.arc(x + r, y + r, r, 2.0 * (PI / 2.), 3. * (PI / 2.));
    ct.arc(w - r, y + r, r, 3.0 * (PI / 2.), 4. * (PI / 2.));
    ct.arc(w - r, h - r, r, 0.0 * (PI / 2.), 1. * (PI / 2.));
    ct.arc(x + r, h - r, r, 1.0 * (PI / 2.), 2. * (PI / 2.));
    ct.close_path();
}

fn draw_gauge(area: &gtk::DrawingArea, ct: &Context, width: i32, height: i32) {
    let parent = area.ancestor(super::Gauge::static_type()).unwrap();

    draw_background(area, ct, width, height);

    draw_current(ct, parent, width as f64, height as f64);

    ct.fill().unwrap_or_default();
}

fn draw_current(ct: &Context, parent: gtk::Widget, width: f64, height: f64) {
    let cur: f64 = parent.property_value("current").get::<f32>().unwrap_or(0.0) as f64;
    let min: f64 = parent.property_value("min").get::<f32>().unwrap_or(0.0) as f64;
    let max: f64 = parent.property_value("max").get::<f32>().unwrap_or(0.0) as f64;

    let percentage: f64 = cur / (max - min);
    let upper_bound: f64 = height - (height * percentage);
    let offset: f64 = ((width + height) / 2.0) * 0.01;

    rounded_rect(
        ct,
        offset,
        offset + upper_bound,
        width - offset,
        height - offset,
        5.0,
    );

    ct.set_source_rgb(1., percentage, 0.);
    ct.fill().unwrap_or_default();
}
