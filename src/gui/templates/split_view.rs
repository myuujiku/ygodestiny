use adw::prelude::*;
use relm4::prelude::*;
use relm4_icons::icon_name;

#[relm4::widget_template(pub)]
impl WidgetTemplate for SplitView {
    view! {
        adw::NavigationSplitView {
            connect_collapsed_notify[top_search_entry, bottom_search_entry] => move |split_view| {
                let collapsed = split_view.is_collapsed();
                top_search_entry.set_visible(!collapsed);
                bottom_search_entry.set_visible(collapsed);
            },

            set_sidebar_width_fraction: 0.3,
            set_min_sidebar_width: 240.0,
            set_max_sidebar_width: 720.0,

            #[wrap(Some)]
            set_sidebar = &adw::NavigationPage {
                set_title: "Game Modes",

                adw::ToolbarView {
                    add_top_bar = &adw::HeaderBar {
                        #[name = "add_button"]
                        pack_start = &gtk::Button {
                            set_icon_name: icon_name::PLUS_LARGE,
                        },
                    },

                    #[wrap(Some)]
                    set_content = &gtk::ScrolledWindow {
                        set_hscrollbar_policy: gtk::PolicyType::Never,

                        #[name = "sidebar_content"]
                        adw::Bin,
                    },

                    #[name = "top_search_entry"]
                    add_top_bar = &get_search_entry() -> adw::Bin {},

                    #[name = "bottom_search_entry"]
                    add_bottom_bar = &get_search_entry() -> adw::Bin {
                        set_visible: false,
                    },
                }
            },

            #[wrap(Some)]
            set_content = &adw::NavigationPage {
                set_title: "Content",

                #[name = "content_view"]
                adw::Bin {
                    adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {
                            set_show_title: false,
                        },

                        #[wrap(Some)]
                        set_content = &adw::StatusPage {
                            set_title: "Select a Game Mode",
                            set_icon_name: Some(icon_name::NINTENDO_CONTROLLER),
                        },
                    }
                }
            },
        }
    }
}

fn get_search_entry() -> adw::Bin {
    relm4::view! {
        bin = adw::Bin {
            add_css_class: "toolbar",
            gtk::SearchEntry {
                set_placeholder_text: Some("Search Game Modes"),
            },
        }
    };

    bin
}
