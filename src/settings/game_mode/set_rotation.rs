use adw::prelude::*;
use relm4::{adw, gtk};

#[derive(Debug)]
pub struct Setting {
    pub keep_sets: usize,
    pub exclude_first: usize,
    pub full_rotation: bool,
}

pub struct Widgets {
    pub root: adw::ExpanderRow,
    keep_sets: adw::SpinRow,
    exclude_first: adw::SpinRow,
    full_rotation: adw::SwitchRow,
}

impl Widgets {
    pub fn build() -> Self {
        let root = adw::ExpanderRow::builder()
            .show_enable_switch(true)
            .title("Set rotation")
            .subtitle("Remove pulled cards from the card pool after some rounds")
            .expanded(false)
            .enable_expansion(false)
            .build();

        let keep_sets_adjustment = gtk::Adjustment::builder()
            .lower(0.0)
            .step_increment(1.0)
            .upper(100.0)
            .build();

        let keep_sets = adw::SpinRow::builder()
            .title("Rotation rounds")
            .subtitle("Number of rounds to keep sets for")
            .adjustment(&keep_sets_adjustment)
            .build();

        let exclude_first_adjustment = gtk::Adjustment::builder()
            .lower(0.0)
            .step_increment(1.0)
            .upper(100.0)
            .build();

        let exclude_first = adw::SpinRow::builder()
            .title("Rotation delay")
            .subtitle("Number of rounds to exclude from rotation")
            .adjustment(&exclude_first_adjustment)
            .build();

        let full_rotation = adw::SwitchRow::builder()
            .title("Full rotation")
            .subtitle("Remove all sets from the pool at once instead of one round at a time")
            .build();

        root.add_row(&keep_sets);
        root.add_row(&exclude_first);
        root.add_row(&full_rotation);

        Self {
            root,
            keep_sets,
            exclude_first,
            full_rotation,
        }
    }

    pub fn get(&self) -> &adw::ExpanderRow {
        &self.root
    }

    pub fn load(&self, setting: &Option<Setting>) {
        if let Some(setting) = setting {
            self.root.set_enable_expansion(true);
            self.keep_sets.set_value(setting.keep_sets as f64);
            self.exclude_first.set_value(setting.exclude_first as f64);
            self.full_rotation.set_active(setting.full_rotation);
        }
    }

    pub fn collect(&self) -> Option<Setting> {
        match self.root.enables_expansion() {
            true => Some(Setting {
                keep_sets: self.keep_sets.value() as usize,
                exclude_first: self.exclude_first.value() as usize,
                full_rotation: self.full_rotation.is_active(),
            }),
            false => None,
        }
    }
}
