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

pub fn editor_scene<'a>(app: &App) -> Element<'a, Message> {
    let left_column = left_column(app);

    let tasks_table =
        create_tasks_table(&app.table, true, app.tape.get_current_char(), DEFAULT_STATE);

    let tape_preview = create_tape_preview(&app.tape);

    scene_frame(tape_preview.into(), left_column.into(), tasks_table.into())
}

fn left_column<'a>(app: &App) -> Element<'a, Message> {
    let initial_tape_input = text_input(
        "Set initial tape...",
        app.tape.get_chars_without_margin(),
        Message::TapeInputCharsChanged,
    )
    .padding(10)
    .size(20);

    let initial_cursor_position_input = numeric_input(
        "Set initial cursor position...",
        app.tape.get_cursor_position(),
        Message::TapeInputCursorPositionChanged,
    );

    let table_characters_input = text_input(
        "Set table characters...",
        &app.table.get_characters(),
        &Message::TableCharactersChanged,
    )
    .padding(10)
    .size(20);

    let table_states_number_input = numeric_input(
        "Set table states number...",
        app.table.get_states_number(),
        Message::TableStatesNumberChanged,
    )
    .can_be_negative(false);

    let start_button = button("Start")
        .padding(10)
        .width(Length::Fill)
        .on_press(Message::MachineStarted);

    let new_file_button = button("New file")
        .padding(10)
        .width(Length::Fill)
        .on_press(Message::NewFileClicked);

    let open_file_button = button("Open file")
        .padding(10)
        .width(Length::Fill)
        .on_press(Message::OpenFileClicked);

    let save_file_button = button("Save file")
        .padding(10)
        .width(Length::Fill)
        .on_press(Message::SaveFileClicked);

    let save_file_as_button = button("Save file as")
        .padding(10)
        .width(Length::Fill)
        .on_press(Message::SaveFileAsClicked);

    ui_column![
        "Tape text",
        initial_tape_input,
        "Cursor position",
        initial_cursor_position_input,
        "Table states number",
        table_states_number_input,
        "Table characters",
        table_characters_input,
        start_button,
        new_file_button,
        open_file_button,
        save_file_button,
        save_file_as_button
    ]
    .width(Length::Units(200))
    .spacing(10)
    .into()
}
