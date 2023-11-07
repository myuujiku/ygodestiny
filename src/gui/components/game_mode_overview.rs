use adw::prelude::*;
use gtk::glib;
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;

use crate::data::{
    game_mode::{GameMode, GameModeMetadata},
    Metadata, RedbStorage,
};
use crate::gui::components::new_game_mode_page;
use crate::gui::factories::game_mode_entry;
use crate::gui::templates::{breakpoint, SplitView};

pub struct Component {
    game_mode_entries: FactoryVecDeque<game_mode_entry::Component>,
}

#[derive(Debug)]
pub enum Input {
    New,
    ClosePage,
    Open(u128),
    Update,
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

                        #[template_child]
                        sidebar_content {
                            #[local_ref]
                            game_mode_entry_box -> gtk::ListBox {
                                add_css_class: "navigation-sidebar",
                            }
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
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let game_mode_entries = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .forward(sender.input_sender(), |output| match output {
                game_mode_entry::Output::Open(uuid) => Input::Open(uuid),
            });

        let model = Self { game_mode_entries };

        let game_mode_entry_box = model.game_mode_entries.widget();

        let widgets = view_output!();

        sender.input(Input::Update);

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
                        Output::Created(uuid) => sender.input(Input::Open(uuid)),
                        Output::Exit => sender.input(Input::ClosePage),
                    }
                });
            }
            Input::ClosePage => {
                widgets.navigation_view.pop();
            }
            Input::Open(uuid) => {
                widgets.navigation_view.pop();
                println!("{uuid}");
            }
            Input::Update => {
                let mut all = GameMode::get_all_as::<Metadata<GameModeMetadata>>().unwrap();
                all.sort_unstable_by(|a, b| {
                    b.1.metadata.last_played.cmp(&a.1.metadata.last_played)
                });

                let mut guard = self.game_mode_entries.guard();
                guard.clear();

                for data in all {
                    guard.push_back(game_mode_entry::Data {
                        uuid: data.0,
                        name: data.1.metadata.name,
                        description: data.1.metadata.description,
                    });
                }
            }
        }
    }
}
