pub mod multi_choice;
pub mod set_rotation;

use serde::{Deserialize, Serialize};

macro_rules! create_settings {
    ( $( $i:ident ),* ) => {
        #[derive(Debug, Default, Serialize, Deserialize)]
        pub struct Settings {
            $( pub $i: Option<$i::Setting>, )*
        }
    }
}

create_settings!(multi_choice, set_rotation);

macro_rules! create_settings_component {
    (
        # $t:literal
        | $( $l:literal $( $i:ident ),* )|*
    ) => {
        use adw::prelude::*;
        use relm4::prelude::*;

        use crate::data::game_mode::{GameMode, GameModeMetadata};
        use crate::gui::templates::Clamp;
        use crate::settings::game_mode::Settings;

        pub struct Component {
            $( $( $i: $i::Widgets, )* )*
        }

        #[relm4::component(pub)]
        impl relm4::Component for Component {
            type Init = Option<Settings>;
            type Input = ();
            type Output = GameMode;
            type CommandOutput = ();
            type Widgets = Widgets;

            view! {
                adw::ToolbarView {
                    #[wrap(Some)]
                    set_content = &adw::StatusPage {
                        set_title: "Game Mode settings",

                        #[template]
                        #[wrap(Some)]
                        set_child = &Clamp {
                            gtk::Box::new(gtk::Orientation::Vertical, 12) {
                                adw::PreferencesGroup {
                                    #[name = "__name_row"]
                                    adw::EntryRow {
                                        set_title: "Name",
                                    },
                                },

                                $( adw::PreferencesGroup {
                                    set_title: $l,

                                    $(
                                        add = model.$i.get(),
                                    )*
                                }, )*
                            },
                        },
                    },

                    add_bottom_bar = &gtk::ActionBar {
                        pack_end = &gtk::Button {
                            add_css_class: "suggested-action",
                            set_label: "Confirm",
                            connect_clicked => (),
                        },
                    },
                }
            }

            fn init(
                settings: Self::Init,
                _root: &Self::Root,
                _sender: ComponentSender<Self>,
            ) -> ComponentParts<Self> {
                let model = Self {
                    $( $( $i: $i::Widgets::build(), )* )*
                };

                if let Some(settings) = settings {
                    $( $( model.$i.load(&settings.$i); )* )*
                }

                let widgets = view_output!();

                ComponentParts { model, widgets }
            }

            fn update_with_view(
                &mut self,
                widgets: &mut Self::Widgets,
                _: (),
                sender: ComponentSender<Self>,
                _root: &Self::Root,
            ) {
                sender.output(
                    GameMode {
                        #[allow(clippy::needless_update)]
                        settings: Settings {
                            $( $( $i: self.$i.collect(), )* )*
                            ..Default::default()
                        },
                        metadata: GameModeMetadata {
                            name: widgets.__name_row.text().to_string(),
                            game_mode_type: $t.to_string(),
                            last_played: std::time::SystemTime::now(),
                        },
                        games: Vec::new(),
                    }
                ).expect("Failed to output")
            }
        }
    }
}

pub mod components;
