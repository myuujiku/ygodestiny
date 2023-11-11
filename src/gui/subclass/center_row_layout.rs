use gtk::glib;
use ygodestiny_macros::object_subclass;

object_subclass! {
    CenterRowLayout(LayoutManager);

    imports = [ gtk::Widget ];

    impl LayoutManager {
        fn request_mode(&self, _: &Widget) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::ConstantSize
        }

        fn measure(
            &self,
            widget: &Widget,
            orientation: gtk::Orientation,
            _for_size: i32,
        ) -> (i32, i32, i32, i32) {
            let mut n_children = 0;
            let mut child_width = 0;
            let mut child_height = 0;

            let mut current_child = widget.first_child();
            while let Some(child) = current_child {
                n_children += 1;
                let (min_size, _) = child.preferred_size();
                child_width = child_width.max(min_size.width());
                child_height = child_height.max(min_size.height());

                current_child = child.next_sibling();
            }

            let children_per_row = 1 + (widget.width() - child_width).max(0)
                / child_width;

            let (req, pref) = match orientation {
                gtk::Orientation::Horizontal => (
                    child_width,
                    n_children * child_width,
                ),
                gtk::Orientation::Vertical => (
                    child_height,
                    n_children / children_per_row * child_height
                ),
                _ => unreachable!(),
            };

            (req, pref, -1, -1)
        }

        fn allocate(
            &self,
            widget: &Widget,
            _: i32,
            _: i32,
            _: i32,
        ) {
            let mut n_children = 0;
            let mut child_width = 0;
            let mut child_height = 0;

            let mut current_child = widget.first_child();
            while let Some(child) = current_child {
                let (min_size, _) = child.preferred_size();
                child_width = child_width.max(min_size.width());
                child_height = child_height.max(min_size.height());
                n_children += 1;

                current_child = child.next_sibling();
            }

            let col_count = 1 + (widget.width() - child_width) / child_width;
            let col_count_last_row = n_children % col_count;
            let full_rows = n_children / col_count;
            let unneeded_space = (widget.width() - col_count * child_width) / 2;

            let mut current_child = widget.first_child();
            for row in 0..full_rows {
                for col in 0..col_count {
                    let child = current_child.as_ref()
                        .expect("Counted children incorrectly");

                    child.size_allocate(&gtk::Allocation::new(
                        unneeded_space + col * child_width,
                        row * child_height,
                        child_width,
                        child_height,
                    ), -1);

                    current_child = child.next_sibling();
                }
            }

            let offset = (widget.width() - col_count_last_row * child_width) / 2;
            for col in 0..col_count_last_row {
                let child = current_child.as_ref()
                    .expect("Counted children incorrectly");

                child.size_allocate(&gtk::Allocation::new(
                    offset + col * child_width,
                    full_rows * child_height,
                    child_width,
                    child_height,
                ), -1);

                current_child = child.next_sibling();
            }
        }
    }
}

impl CenterRowLayout {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
