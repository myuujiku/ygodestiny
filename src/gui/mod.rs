pub mod components;
pub mod factories;
pub mod subclass;
pub mod templates;

use adw::prelude::*;
use gtk::gdk;
use relm4::prelude::*;

use components::game_mode_overview;

pub struct Main {
    game_mode_overview: Controller<game_mode_overview::Component>,
}

#[derive(Debug)]
pub struct Input {}

#[relm4::component(pub)]
impl Component for Main {
    type Init = ();
    type Input = Input;
    type Output = ();
    type Widgets = Widgets;
    type CommandOutput = ();

    view! {
        adw::Window {
            set_default_width: 800,
            set_default_height: 600,
            //add_css_class: "devel",

            model.game_mode_overview.widget(),
        }
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        load_css();
        relm4_icons::initialize_icons();

        let game_mode_overview = game_mode_overview::Component::builder()
            .launch(())
            .forward(sender.input_sender(), |_| Input {});

        let model = Self { game_mode_overview };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

fn load_css() {
    let provider = gtk::CssProvider::new();

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to default display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
