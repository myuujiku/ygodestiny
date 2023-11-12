use adw::prelude::*;
use relm4::prelude::*;
use relm4_icons::icon_name;

use crate::data::{game_mode::GameMode, RedbStorage};

pub struct Component {
    uuid: u128,
    game_mode: GameMode,
}

#[derive(Debug)]
pub enum Input {
    Edit,
    Copy,
    Delete,
}

#[derive(Debug)]
pub enum Output {
    Edit(u128),
    Copy(u128),
    Delete(u128),
}

#[relm4::component(pub)]
impl SimpleComponent for Component {
    type Init = u128;
    type Input = Input;
    type Output = Output;
    type Widgets = Widgets;

    view! {
        adw::ToolbarView {
            add_top_bar = &adw::HeaderBar {
                set_show_title: false,

                pack_end = &gtk::MenuButton {
                    set_icon_name: icon_name::MENU_LARGE,

                    #[wrap(Some)]
                    set_popover = &gtk::Popover {
                        connect_show => |popover| {
                            popover.unset_state_flags(gtk::StateFlags::PRELIGHT);
                        },

                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 2,

                            gtk::Button {
                                add_css_class: "flat",

                                gtk::Label {
                                    set_label: "Edit",
                                    add_css_class: "body",
                                    set_halign: gtk::Align::Start,
                                },

                                connect_clicked => Input::Edit,
                            },

                            gtk::Button {
                                add_css_class: "flat",

                                gtk::Label {
                                    set_label: "Copy",
                                    add_css_class: "body",
                                    set_halign: gtk::Align::Start,
                                },

                                connect_clicked => Input::Copy,
                            },

                            gtk::Separator::new(gtk::Orientation::Horizontal) {
                                set_margin_top: 5,
                                set_margin_bottom: 5,
                            },

                            gtk::Button {
                                add_css_class: "flat",
                                add_css_class: "error",

                                gtk::Label {
                                    set_label: "Delete",
                                    add_css_class: "body",
                                    set_halign: gtk::Align::Start,
                                },

                                connect_clicked => Input::Delete,
                            },
                        },
                    },
                },
            },

            #[wrap(Some)]
            set_content = &adw::StatusPage {
                set_title: &model.game_mode.metadata.name,
                set_description: Some(&model.game_mode.metadata.game_mode_type),

                gtk::Button {
                    set_label: "Play",
                    add_css_class: "suggested-action",
                    add_css_class: "pill",
                    set_halign: gtk::Align::Center,
                },
            },
        },
    }

    fn init(
        uuid: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            uuid,
            game_mode: GameMode::get(uuid).unwrap(),
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            Input::Edit => sender
                .output(Output::Edit(self.uuid))
                .expect("Failed to output from game_mode_page"),
            Input::Copy => sender
                .output(Output::Copy(self.uuid))
                .expect("Failed to output from game_mode_page"),
            Input::Delete => sender
                .output(Output::Delete(self.uuid))
                .expect("Failed to output from game_mode_page"),
        }
    }
}
