use adw::prelude::*;
use relm4::prelude::*;

use crate::data::game_mode::GameMode;
use crate::data::RedbStorage;
use crate::gui::templates::NewGameMode;
use crate::settings::game_mode::components::timeline;

pub struct Component;

#[derive(Debug)]
pub enum Input {
    Done(GameMode),
    GoNext,
    GoBack,
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

            #[template]
            #[name = "content"]
            NewGameMode {
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
        _root: &Self::Root,
    ) {
        match msg {
            Input::Done(game_mode) => {
                // TODO: Error handling
                let uuid = GameMode::generate_uuid().unwrap().unwrap();
                game_mode.save(uuid).unwrap();
                sender
                    .output(Output::Created(uuid))
                    .expect("Failed to output from new_game_mode_page");
            }
            Input::GoNext => {
                widgets
                    .content
                    .carousel
                    .scroll_to(&widgets.content.settings, true);
            }
            Input::GoBack => match widgets.content.carousel.position().round() as usize {
                0 => sender
                    .output(Output::Exit)
                    .expect("Failed to output new_game_mode_page"),
                1 => widgets
                    .content
                    .carousel
                    .scroll_to(&widgets.content.types, true),
                _ => (),
            },
            Input::SelectTimeline => {
                let page = timeline::Component::builder().launch(None);
                widgets.content.settings.set_child(Some(page.widget()));

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
