use crate::settings::game_mode::{multi_choice, set_rotation};

create_settings_component! {
    # "Timeline"
    | "General" set_rotation
    | "Draft" multi_choice
}
