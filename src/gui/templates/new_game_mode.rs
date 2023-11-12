use adw::prelude::*;
use relm4::prelude::*;

use crate::gui::templates::Clamp;

#[relm4::widget_template(pub)]
impl WidgetTemplate for NewGameMode {
    view! {
        adw::ToolbarView {
            #[name = "header_bar"]
            add_top_bar = &adw::HeaderBar {
                set_show_back_button: false,

                #[name = "back_button"]
                pack_start = &gtk::Button {
                    set_icon_name: "go-previous",
                },
            },

            #[wrap(Some)]
            #[name = "carousel"]
            set_content = &adw::Carousel {
                set_interactive: false,

                #[name = "types"]
                adw::Bin {
                    set_hexpand: true,

                    #[template]
                    Clamp {
                        adw::StatusPage {
                            set_hexpand: true,
                            set_title: "Select a Game Mode type",

                            #[wrap(Some)]
                            set_child = &adw::PreferencesGroup {
                                #[name = "type_timeline"]
                                adw::ActionRow {
                                    set_title: "Timeline",
                                    set_subtitle: "Progress through sets in a fixed order",
                                    set_activatable: true,
                                    add_suffix: &gtk::Image::builder().icon_name("go-next").build(),
                                },

                                adw::ActionRow {
                                    set_title: "Paths (WIP)",
                                    set_subtitle: "Travel paths that lead to different sets",
                                },

                                adw::ActionRow {
                                    set_title: "Chaos (WIP)",
                                    set_subtitle: "Try to make the best out of a pile of random packs",
                                },

                                adw::ActionRow {
                                    set_title: "Shop (WIP)",
                                    set_subtitle: "Buy packs with Duel Credits",
                                },
                            },
                        },
                    },
                },

                #[name = "settings"]
                adw::Bin {
                    set_hexpand: true,
                },
            },
        }
    }
}
