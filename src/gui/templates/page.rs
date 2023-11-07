use relm4::prelude::*;

#[relm4::widget_template(pub)]
impl WidgetTemplate for Page {
    view! {
        adw::NavigationPage {
            #[name = "toolbar_view"]
            adw::ToolbarView {
                // set_top_bar_style: adw::ToolbarStyle::Raised,

                #[name = "header_bar"]
                add_top_bar = &adw::HeaderBar {},

                #[wrap(Some)]
                set_content = &adw::Bin {},
            }
        }
    }
}
