mod editor_scene;
mod machine_scene;

use crate::{App, Message};
use iced::{
    widget::{column as ui_column, container, row},
    Element, Length,
};

use self::{editor_scene::editor_scene, machine_scene::machine_scene};

pub enum Scene {
    Editor,
    Machine,
}

impl Scene {
    pub fn view<'a>(&self, app: &'a App) -> Element<'a, Message> {
        match self {
            Self::Editor => editor_scene(app),
            Self::Machine => machine_scene(app),
        }
    }
}

pub fn scene_frame<'a>(
    top: Element<'a, Message>,
    left: Element<'a, Message>,
    right: Element<'a, Message>,
) -> Element<'a, Message> {
    let content = ui_column![top, row![left, right].spacing(40)]
        .spacing(20)
        .padding(40);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
