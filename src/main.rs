mod calendar;
mod calendar_entry;
mod window;

use calendar::Calendar;
use calendar_entry::CalendarEntry;
use gdk::Display;
use gtk::{gdk, gio, glib, Application};
use gtk::{prelude::*, CssProvider};
use window::Window;

const APP_ID: &str = "org.gtk_rs.CompositeTemplates1";

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("templates/variant1/style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
fn main() -> glib::ExitCode {
    Calendar::static_type();
    CalendarEntry::static_type();
    // Register and include resources
    gio::resources_register_include!("composite_templates_1.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}
fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}
