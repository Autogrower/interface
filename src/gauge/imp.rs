use gtk::cairo;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use plotters::prelude::*;
use plotters_cairo::CairoBackend;
use std::cell::Cell;
use std::cell::RefCell;
use std::error::Error;

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
}

impl WidgetImpl for Gauge {
    fn snapshot(&self, snapshot: &gtk::Snapshot) {
        let width = self.obj().width() as u32;
        let height = self.obj().height() as u32;
        if width == 0 || height == 0 {
            return;
        }

        let bounds = gtk::graphene::Rect::new(0.0, 0.0, width as f32, height as f32);
        let cr = snapshot.append_cairo(&bounds);
        let backend = CairoBackend::new(&cr, (width, height)).unwrap();
        self.plot_pdf(backend).unwrap();
    }
}

impl Gauge {
    fn plot_pdf<'a, DB: DrawingBackend + 'a>(
        &self,
        mut backend: DB,
    ) -> Result<(), Box<dyn Error + 'a>> {
        let width = self.obj().width() as i32;
        let height = self.obj().height() as i32;

        let percentage: f32 = self.current.get() / (self.max.get() - self.min.get());

        let upper_bound: i32 = height - (height as f32 * percentage) as i32;

        backend
            .draw_rect((0, upper_bound), (width, height), &CYAN, true)
            .unwrap();

        Ok(())
    }
}
