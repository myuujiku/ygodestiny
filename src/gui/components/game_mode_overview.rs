use adw::prelude::*;
use gtk::glib;
use relm4::prelude::*;

use crate::gui::components::new_game_mode_page;
use crate::gui::templates::{breakpoint, SplitView};

pub struct Component;

#[derive(Debug)]
pub enum Input {
    New,
    Open(u128),
}

#[relm4::component(pub)]
impl relm4::Component for Component {
    type Init = ();
    type Input = Input;
    type Output = ();
    type Widgets = Widgets;
    type CommandOutput = ();

    view! {
        adw::BreakpointBin {
            set_width_request: 400,
            set_height_request: 260,

            #[wrap(Some)]
            #[name = "navigation_view"]
            set_child = &adw::NavigationView {
                adw::NavigationPage {
                    set_title: "Game Modes",

                    #[template]
                    #[name = "split_view"]
                    SplitView {
                        #[template_child]
                        add_button {
                            connect_clicked => Input::New,
                        },
                    },
                },
            },

            add_breakpoint = breakpoint::default() {
                add_setter: (
                    split_view.upcast_ref::<glib::Object>(),
                    "collapsed",
                    &glib::Value::from(true),
                ),
            },
        },
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Widgets,
        msg: Input,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match msg {
            Input::New => {
                let page = new_game_mode_page::Component::builder().launch(());
                widgets.navigation_view.push(page.widget());

                relm4::spawn_local(async move {
                    let output = page
                        .into_stream()
                        .recv_one()
                        .await
                        .expect("Failed to recieve output from new_game_mode_page");

                    use new_game_mode_page::Output;
                    match output {
                        Output::Created(uuid) => {
                            sender.input(Input::Open(uuid));
                        }
                        Output::Exit => (),
                    }
                });
            }
            _ => todo!(),
        }
    }
}
