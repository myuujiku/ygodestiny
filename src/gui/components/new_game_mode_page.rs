use adw::prelude::*;
use relm4::prelude::*;

use crate::gui::templates::NewGameMode;
use crate::settings::game_mode::components::timeline;
use crate::settings::game_mode::Settings;

pub struct Component;

#[derive(Debug)]
pub enum Input {
    Done(Settings),
    GoNext,
    GoBack,
    PageChanged(u32),
    SelectTimeline,
}

#[derive(Debug)]
pub enum Output {
    Created(u128),
    Exit,
}

#[relm4::component(pub)]
impl relm4::Component for Component {
    type Init = ();
    type Input = Input;
    type Output = Output;
    type CommandOutput = ();
    type Widgets = Widgets;

    view! {
        adw::NavigationPage {
            set_title: "New Game Mode",
            connect_unrealize[sender] => move |_| {
                sender
                    .output(Output::Exit)
                    .expect("game_mode_overview::Component failed to output");
            },

            #[template]
            #[name = "content"]
            NewGameMode {
                #[template_child]
                carousel {
                    connect_page_changed[sender] => move |_, page| {
                        sender.input(Input::PageChanged(page));
                    }
                },

                #[template_child]
                back_button {
                    connect_clicked => Input::GoBack,
                },

                #[template_child]
                type_timeline {
                    connect_activated => Input::SelectTimeline,
                },
            },
        }
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
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
            Input::Done(settings) => {
                println!("{:#?}", settings);
            }
            // This is jank, change it
            Input::GoNext => {
                widgets
                    .content
                    .carousel
                    .scroll_to(&widgets.content.settings, true);
            }
            Input::GoBack => {
                widgets
                    .content
                    .carousel
                    .scroll_to(&widgets.content.types, true);
            }
            Input::PageChanged(page) => {
                widgets.content.header_bar.set_show_back_button(page == 0);
                widgets.content.back_button.set_visible(page == 1);
            }
            Input::SelectTimeline => {
                let page = timeline::Component::builder().launch(());
                widgets.content.settings.set_content(Some(page.widget()));

                {
                    let sender = sender.clone();

                    relm4::spawn_local(async move {
                        let output = page
                            .into_stream()
                            .recv_one()
                            .await
                            .expect("Failed to recieve output from timeline");

                        sender.input(Input::Done(output));
                    });
                }

                sender.input(Input::GoNext);
            }
        }
    }
}
