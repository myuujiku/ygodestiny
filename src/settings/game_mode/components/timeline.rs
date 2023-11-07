use crate::settings::game_mode::{multi_choice, set_rotation};

create_settings_component! {
    | "General" set_rotation
    | "Draft" multi_choice
}
