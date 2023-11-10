use adw::prelude::*;
use gtk::{gio, glib};

use ygodestiny::data::dirs;
use ygodestiny::gui;

fn main() -> anyhow::Result<glib::ExitCode> {
    env_logger::init();
    dirs::init()?;

    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(run_app);

    Ok(app.run())
}

fn run_app(app: &adw::Application) {}
