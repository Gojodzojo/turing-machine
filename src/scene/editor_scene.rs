use crate::{
    constants::DEFAULT_STATE, numeric_input::numeric_input,
    table::create_tasks_table::create_tasks_table, tape::create_tape_preview::create_tape_preview,
    App, Message,
};
use iced::{
    widget::{button, column as ui_column, text_input},
    Element, Length,
};

use super::scene_frame;

pub fn editor_scene<'a>(app: &'a App) -> Element<'a, Message> {
    let left_column = left_column(app);

    let tasks_table = create_tasks_table(
        &app.table,
        true,
        app.tape.get_current_char(),
        DEFAULT_STATE,
        &app.focused_widget,
    );

    let tape_preview = create_tape_preview(&app.tape);

    scene_frame(tape_preview.into(), left_column.into(), tasks_table.into())
}

fn left_column<'a>(app: &'a App) -> Element<'a, Message> {
    let initial_tape_input = text_input(
        app.language.initial_tape_input_placeholder,
        app.tape.get_chars_without_margin(),
        Message::TapeInputCharsChanged,
    )
    .padding(10)
    .size(20);

    let initial_cursor_position_input = numeric_input(
        app.language.initial_cursor_position_input_placeholder,
        app.tape.get_cursor_position(),
        &app.focused_widget,
        Message::TapeInputCursorPositionChanged,
    );

    let tape_length_input = numeric_input(
        app.language.tape_length_input_placeholder,
        app.tape.get_length(),
        &app.focused_widget,
        Message::TapeLengthChanged,
    )
    .can_be_negative(false);

    let table_characters_input = text_input(
        app.language.table_characters_input_placeholder,
        &app.table.get_characters(),
        &Message::TableCharactersChanged,
    )
    .padding(10)
    .size(20);

    let table_states_number_input = numeric_input(
        app.language.table_states_number_input_placeholder,
        app.table.get_states_number(),
        &app.focused_widget,
        Message::TableStatesNumberChanged,
    )
    .can_be_negative(false);

    let start_button = button(app.language.start_machine_button_text)
        .padding(10)
        .width(Length::Fill)
        .on_press(Message::MachineStarted);

    ui_column![
        app.language.initial_tape_input_label,
        initial_tape_input,
        app.language.tape_length_input_label,
        tape_length_input,
        app.language.initial_cursor_position_input_label,
        initial_cursor_position_input,
        app.language.table_states_number_input_label,
        table_states_number_input,
        app.language.table_characters_input_label,
        table_characters_input,
        start_button,
    ]
    .width(Length::Units(200))
    .spacing(10)
    .into()
}
