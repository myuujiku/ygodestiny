use adw::prelude::WidgetExt;
use gtk::{glib, prelude::IsA};
use ygodestiny_macros::object_subclass;

use super::CenterRowLayout;

object_subclass! {
    CardBox(Widget);

    impl Object {
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
}

impl CardBox {
    pub fn new() -> Self {
        let obj: Self = glib::Object::new();
        obj.set_layout_manager(Some(CenterRowLayout::new()));

        obj
    }

    pub fn add_child<W: IsA<gtk::Widget>>(&self, widget: &W) {
        widget.set_parent(self);
    }
}
