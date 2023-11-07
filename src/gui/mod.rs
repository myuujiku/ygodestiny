pub mod components;
pub mod templates;

use adw::prelude::*;
use gtk::{gdk, glib};
use relm4::prelude::*;

use templates::breakpoint;
use templates::SplitView;

pub struct Main;

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
            set_width_request: 360,
            set_height_request: 240,
            add_css_class: "devel",

            #[template]
            #[name = "split_view"]
            SplitView {},

            add_breakpoint = breakpoint::default() {
                add_setter: (
                    split_view.upcast_ref::<glib::Object>(),
                    "collapsed",
                    &glib::Value::from(true)
                ),
            },
        }
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        load_css();
        relm4_icons::initialize_icons();

        let model = Self;
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
