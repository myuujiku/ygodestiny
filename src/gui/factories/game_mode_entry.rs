use adw::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
pub struct Component {
    uuid: u128,
}

#[derive(Debug)]
pub struct Data {
    pub uuid: u128,
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub enum Input {
    Open,
    SetVisible(bool),
    SetText(String, String),
}

#[derive(Debug)]
pub enum Output {
    Open(u128),
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
            set_selectable: true,
            set_activatable: true,
            connect_activated => Input::Open,
        },
    }

    fn init_model(data: Data, _: &Self::Index, sender: FactorySender<Self>) -> Self {
        sender.input(Input::SetText(data.name, data.description));

        Self { uuid: data.uuid }
    }

    fn update_with_view(&mut self, widgets: &mut Widgets, msg: Input, sender: FactorySender<Self>) {
        match msg {
            Input::Open => sender
                .output(Output::Open(self.uuid))
                .expect("Failed to output from game_mode_entry factory"),
            Input::SetVisible(value) => widgets.root.set_visible(value),
            Input::SetText(name, description) => {
                widgets.root.set_title(&name);
                widgets.root.set_subtitle(&description);
            }
        }
    }
}
