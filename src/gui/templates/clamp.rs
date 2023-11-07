use adw::prelude::*;
use relm4::prelude::*;

#[relm4::widget_template(pub)]
impl WidgetTemplate for Clamp {
    view! {
        adw::Clamp {
            set_orientation: gtk::Orientation::Horizontal,
            set_maximum_size: 1200,
            set_unit: adw::LengthUnit::Sp,
        }
    }
}
