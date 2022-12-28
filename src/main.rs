mod constants;
mod machine;
mod number_input;
mod table;
mod tape;
mod task;

use constants::MAX_TAPE_CHARS_NUMBER;
use iced::widget::{column as ui_column, container, row, text_input};
use iced::{Element, Length, Sandbox, Settings};
use machine::Machine;
use number_input::number_input;
use table::{table_tasks_editor::table_tasks_editor, Table};
use tape::create_tape_preview::create_tape_preview;
use tape::Tape;
use task::Task;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

struct App {
    table: Table,
    machine: Machine,
    tape: Tape,
}

#[derive(Debug, Clone)]
enum Message {
    TapeInputCharsChanged(String),
    TapeInputCursorPositionChanged(isize),
    TableCharactersChanged(String),
    TableStatesNumberChanged(usize),
    TableTaskChanged(Task, usize, usize),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            table: Table::new_empty(),
            machine: Machine::new(),
            tape: Tape::new(),
        }
    }

    fn title(&self) -> String {
        "Turing Machine".into()
    }

    fn update(&mut self, message: Self::Message) {
        use Message::*;

        match message {
            TapeInputCharsChanged(new_chars) => self.tape.set_input_chars(new_chars),
            TableTaskChanged(task, row, column) => self.table.set_task(task, row, column),
            TableCharactersChanged(new_characters) => self.table.set_characters(new_characters),
            TapeInputCursorPositionChanged(position) => {
                self.tape.set_input_cursor_position(position)
            }
            TableStatesNumberChanged(new_states_number) => {
                self.table.set_states_number(new_states_number)
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let initial_tape_input = text_input(
            "Set initial tape...",
            self.tape.get_input_chars(),
            Message::TapeInputCharsChanged,
        )
        .padding(10)
        .size(20);

        let initial_cursor_position_input = number_input(
            "Set initial cursor position...",
            self.tape.get_input_cursor_position(),
            &Message::TapeInputCursorPositionChanged,
        );

        let table_characters_input = text_input(
            "Set table characters...",
            &self.table.get_characters(),
            &Message::TableCharactersChanged,
        )
        .padding(10)
        .size(20);

        let table_states_number_input = number_input(
            "Set table states number...",
            self.table.get_states_number(),
            &Message::TableStatesNumberChanged,
        );

        let tasks_editor = table_tasks_editor(&self.table, &Message::TableTaskChanged);

        let content = ui_column![
            create_tape_preview(&self.tape),
            row![
                ui_column![
                    "Tape text",
                    initial_tape_input,
                    "Cursor position",
                    initial_cursor_position_input,
                    "Table states number",
                    table_states_number_input,
                    "Table characters",
                    table_characters_input,
                ]
                .max_width(200)
                .spacing(10),
                tasks_editor
            ]
            .spacing(20)
            .padding(20)
        ]
        .align_items(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
