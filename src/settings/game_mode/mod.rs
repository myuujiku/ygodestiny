pub mod multi_choice;
pub mod set_rotation;

macro_rules! create_settings {
    ( $( $i:ident ),* ) => {
        #[derive(Debug, Default)]
        pub struct Settings {
            $( pub $i: Option<$i::Setting>, )*
        }
    }
}

create_settings!(multi_choice, set_rotation);

macro_rules! create_settings_component {
    ( | $( $l:literal $( $i:ident ),* )|* ) => {
        use adw::prelude::*;
        use relm4::prelude::*;

        use crate::settings::game_mode::Settings;

        pub struct Component {
            $( $( $i: $i::Widgets, )* )*
        }

        #[relm4::component(pub)]
        impl SimpleComponent for Component {
            type Init = Option<Settings>;
            type Input = ();
            type Output = Settings;
            type Widgets = Widgets;

            view! {
                adw::ToolbarView {
                    #[wrap(Some)]
                    set_content = &adw::StatusPage {
                        set_title: "Game Mode settings",

                        #[wrap(Some)]
                        set_child = &gtk::Box::new(gtk::Orientation::Vertical, 12) {
                            $( adw::PreferencesGroup {
                                set_title: $l,

                                $(
                                    add = model.$i.get(),
                                )*
                            }, )*
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

            fn update(
                &mut self,
                _: (),
                sender: ComponentSender<Self>,
            ) {
                sender.output(
                    Settings {
                        $( $( $i: self.$i.collect(), )* )*
                        ..Default::default()
                    }
                ).expect("Failed to output")
            }
        }
    }
}

pub mod components;
