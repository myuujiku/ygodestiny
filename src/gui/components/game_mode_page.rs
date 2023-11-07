use relm4::prelude::*;
use adw::prelude::*;

use crate::data::{game_mode::GameMode, RedbStorage};

pub struct Component {
    uuid: u128,
    game_mode: GameMode,
}

#[derive(Debug)]
pub enum Input {}

#[derive(Debug)]
pub enum Output {}

#[relm4::component(pub)]
impl SimpleComponent for Component {
    type Init = u128;
    type Input = Input;
    type Output = Output;
    type Widgets = Widgets;

    view! {
        adw::ToolbarView {
            #[wrap(Some)]
            set_content = &adw::StatusPage {
                set_title: &model.game_mode.metadata.name,
                set_description: Some(&model.game_mode.metadata.game_mode_type),

                adw::PreferencesGroup {
                    adw::ActionRow {
                        set_title: "Settings",
                        set_activatable: true,
                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                        },
                    },
                },
            },
        },
    }

    fn init(
        uuid: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { uuid, game_mode: GameMode::get(uuid).unwrap() };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
