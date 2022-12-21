use iced::widget::{text_input, Row, TextInput};

use crate::number_input::number_input;

use super::Table;

pub fn table_states_number_input<'a, Message: 'a + Clone>(
    table: &Table,
    on_number_change: &'a impl Fn(usize) -> Message,
) -> Row<'a, Message> {
    number_input(
        "Set table states number...",
        table.states_number,
        Some(1),
        on_number_change,
    )
}
