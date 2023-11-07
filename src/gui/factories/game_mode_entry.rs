use std::time::SystemTime;

use adw::prelude::*;
use relm4::prelude::*;

use crate::data::game_mode::GameModeMetadata;

#[derive(Debug)]
pub struct Collection {
    uuid: u128,
    name: String,
    description: String,
    last_played: SystemTime,
    index: DynamicIndex,
}

#[derive(Debug)]
pub struct Data {
    pub uuid: u128,
    pub metadata: GameModeMetadata,
}

#[derive(Debug)]
pub enum Input {
    Open,
    SetVisible(bool),
}

#[derive(Debug)]
pub enum Output {
    Output(u128),
}

#[relm4::factory(pub)]
impl FactoryComponent for Component {
    type Init = Data;
    type Input = Input;
    type Output = Output;
    type CommandOutput = ();
    type Widgets = Widgets;
    type ParentWidget = gtk::ListBox;
    type Index = DynamicIndex;

    view! {
        #[root]
        #[name = "root"]
        adw::ActionRow {
            set_selectable: false,
            set_title: &self.name,
            set_subtitle: &self.description,
            set_activatable: true,
            connect_activated => Input::Open,
        },
    }

    fn init_model(data: Data, index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        Self {
            uuid: data.uuid,
            name: data.metadata.name,
            description: data.metadata.description,
            last_played: data.metadata.last_played,
            index: index.clone(),
        }
    }

    fn update_with_view(&mut self, widgets: &mut Widgets, msg: Input, sender: FactorySender<Self>) {
        match msg {
            Input::Open => sender
                .output(Output::Open(self.uuid))
                .expect("Failed to output from game_mode_entry factory"),
            Input::SetVisible(value) => widgets.root.set_visible(value),
        }
    }
}
