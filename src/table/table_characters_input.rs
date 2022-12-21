use iced::widget::{text_input, TextInput};

use super::Table;

pub fn table_characters_input<'a, Message: 'a + Clone>(
    table: &Table,
    on_characters_change: &'a impl Fn(String) -> Message,
) -> TextInput<'a, Message> {
    text_input(
        "Set table characters...",
        &table.characters,
        on_characters_change,
    )
    .padding(10)
    .size(20)
}
