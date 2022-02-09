mod application;
mod config;
mod window;
mod sensors;
mod area;

use self::application::SwelterApplication;
use self::window::SwelterWindow;

use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};

use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::gio;
use gtk::prelude::*;
use gtk::glib;
use gtk::glib::{clone, MainContext, PRIORITY_DEFAULT};

fn main() {
    // Set up gettext translations
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    // Load resources
    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/swelter.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);

    // Create a new GtkApplication. The application manages our main loop,
    // application windows, integration with the window manager/compositor, and
    // desktop features such as file opening and single-instance applications.
    let app = SwelterApplication::new("org.gnome.Swelter", &gio::ApplicationFlags::empty());

    // Use glib::MainContext to send messages between hwmons and GtkApplication
    let context = MainContext::default();
    let _guard = context.acquire().unwrap();
    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);

    // Attach receiver signal
    receiver.attach(
        None,
        clone!(@weak app => @default-return Continue(false),
            move |monitors: Vec<sensors::Monitor>| {
                emit_sensors(app, monitors);
                Continue(true)
            }
        )
    );

    // Spawn sensor polling thread before launching running the app
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.spawn(sensors::poll(sender.clone()));

    // Run the application. This function will block until the application
    // exits. Upon return, we have our exit code to return to the shell. (This
    // is the code you see when you do `echo $?` after running a command in a
    // terminal.
    std::process::exit(app.run());
}

fn emit_sensors (app: SwelterApplication, monitors: Vec<sensors::Monitor>) {
    if let Some(window) = app.active_window() {
        let mut index = 0;

        for monitor in monitors.iter() {
            if monitor.sensors.len() == 0 {
                continue;
            }

            for sensor in monitor.sensors.iter() {
                let mut values = Vec::new();

                index += 1;

                // Populate values in a array (index, sensor_name, temperature)
                // Example: ["6", "temp1", "37"]
                values.push(index.to_string());
                values.push(sensor.name.to_string());
                values.push(sensor.temperature.to_string());

                window.emit_by_name_with_values(window::SENSORS_UPDATED_SIGNAL, &[values.to_value()]);
            }
        }
    }
}
