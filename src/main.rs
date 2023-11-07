use adw::prelude::*;
use relm4::prelude::*;

use ygodestiny::data::dirs;
use ygodestiny::gui;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    dirs::init()?;

    let app = relm4::main_adw_application();
    app.set_application_id(Some("xyz.ygodestiny.YGODestiny"));
    app.set_resource_base_path(Some("/xyz/ygodestiny/YGODestiny/"));

    let app = RelmApp::from_app(app);
    app.run::<gui::Main>(());

    Ok(())
}
