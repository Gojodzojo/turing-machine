mod constants;
mod number_input;
mod running_machine;
mod table;
mod task;

use iced::widget::{column as ui_column, container, row, text_input};
use iced::{Element, Length, Sandbox, Settings};
use number_input::number_input;
use running_machine::RunningMachine;
use table::{table_tasks_editor::table_tasks_editor, Table};
use task::Task;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

struct App {
    table: Table,
    initial_tape: String,
    innitial_cursor_position: isize,
    running_machine: RunningMachine,
}

#[derive(Debug, Clone)]
enum Message {
    InitialTapeChanged(String),
    InitialCursorPositionChanged(isize),
    TableCharactersChanged(String),
    TableStatesNumberChanged(usize),
    TableTaskChanged(Task, usize, usize),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        let table = Table::new_empty();
        let initial_tape = String::new();
        let innitial_cursor_position = 0;
        let running_machine = RunningMachine::new(initial_tape.clone(), innitial_cursor_position);

        Self {
            table,
            initial_tape,
            innitial_cursor_position,
            running_machine,
        }
    }

    fn title(&self) -> String {
        "Turing Machine".into()
    }

    fn update(&mut self, message: Self::Message) {
        use Message::*;

        match message {
            TableTaskChanged(task, row, column) => self.table.set_task(task, row, column),
            InitialTapeChanged(new_tape) => self.initial_tape = new_tape,
            TableCharactersChanged(new_characters) => self.table.set_characters(new_characters),
            InitialCursorPositionChanged(position) => self.innitial_cursor_position = position,
            TableStatesNumberChanged(new_states_number) => {
                self.table.set_states_number(new_states_number)
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let initial_tape_input = text_input(
            "Set initial tape...",
            &self.initial_tape,
            Message::InitialTapeChanged,
        )
        .padding(10)
        .size(20);

        let initial_cursor_position_input = number_input(
            "Set initial cursor position...",
            self.innitial_cursor_position,
            None,
            &Message::InitialCursorPositionChanged,
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
            Some(1),
            &Message::TableStatesNumberChanged,
        );

        let tasks_editor = table_tasks_editor(&self.table, &Message::TableTaskChanged);

        let content = row![
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
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
