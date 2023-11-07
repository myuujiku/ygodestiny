use adw::prelude::*;
use relm4::{adw, gtk};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub choices: usize,
    pub selections: usize,
    pub unify_choices: bool,
}

pub struct Widgets {
    pub root: adw::ExpanderRow,
    choices: adw::SpinRow,
    selections: adw::SpinRow,
    unify_choices: adw::SwitchRow,
}

impl Widgets {
    pub fn build() -> Self {
        let root = adw::ExpanderRow::builder()
            .show_enable_switch(true)
            .title("Multi choice")
            .subtitle("Select between multiple packs each draft round")
            .expanded(false)
            .enable_expansion(false)
            .build();

        let choices_adjustment = gtk::Adjustment::builder()
            .lower(2.0)
            .value(2.0)
            .step_increment(1.0)
            .upper(100.0)
            .build();

        let choices = adw::SpinRow::builder()
            .title("Choices")
            .subtitle("Number of packs to choose from")
            .adjustment(&choices_adjustment)
            .build();

        let selections_adjustment = gtk::Adjustment::builder()
            .lower(1.0)
            .value(1.0)
            .step_increment(1.0)
            .upper(1.0)
            .build();

        let selections = adw::SpinRow::builder()
            .title("Selections")
            .subtitle("Number of selections that have to be made")
            .adjustment(&selections_adjustment)
            .build();

        choices_adjustment.connect_value_changed(move |adjmt| {
            let new_val = adjmt.value() - 1.0;
            selections_adjustment.set_upper(new_val);
            if selections_adjustment.value() > new_val {
                selections_adjustment.set_value(new_val);
            }
        });

        let unify_choices = adw::SwitchRow::builder()
            .title("Unify choices")
            .subtitle("Use the same card groups to generate choices (in sets with small groups this might make the same card appear in every choice)")
            .build();

        root.add_row(&choices);
        root.add_row(&selections);
        root.add_row(&unify_choices);

        Self {
            root,
            choices,
            selections,
            unify_choices,
        }
    }

    pub fn get(&self) -> &adw::ExpanderRow {
        &self.root
    }

    pub fn load(&self, setting: &Option<Setting>) {
        if let Some(setting) = setting {
            self.root.set_enable_expansion(true);
            self.choices.set_value(setting.choices as f64);
            self.selections.set_value(setting.selections as f64);
            self.unify_choices.set_active(setting.unify_choices);
        }
    }

    pub fn collect(&self) -> Option<Setting> {
        match self.root.enables_expansion() {
            true => Some(Setting {
                choices: self.choices.value() as usize,
                selections: self.selections.value() as usize,
                unify_choices: self.unify_choices.is_active(),
            }),
            false => None,
        }
    }
}
