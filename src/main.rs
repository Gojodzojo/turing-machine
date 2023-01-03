mod constants;
mod machine;
mod number_input;
mod table;
mod tape;
mod task;

use constants::DEFAULT_STATE;
use iced::widget::{button, column as ui_column, container, row, text, text_input, Column, Row};
use iced::{Element, Length, Sandbox, Settings};
use machine::Machine;
use number_input::number_input;
use table::create_tasks_table::create_tasks_table;
use table::Table;
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
    is_machine_running: bool,
}

#[derive(Debug, Clone)]
enum Message {
    TapeInputCharsChanged(String),
    TapeInputCursorPositionChanged(isize),
    TableCharactersChanged(String),
    TableStatesNumberChanged(usize),
    TableTaskChanged(Task, usize, usize),
    MachineStarted,
    MachineStopped,
    MachineNextStep,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            is_machine_running: false,
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
            TapeInputCharsChanged(new_chars) => self.tape.set_chars(new_chars),
            TapeInputCursorPositionChanged(position) => self.tape.set_cursor_position(position),
            TableTaskChanged(task, row, column) => self.table.set_task(task, row, column),
            TableCharactersChanged(new_characters) => self.table.set_characters(new_characters),
            MachineNextStep => self.machine.next_step(&self.table),
            MachineStopped => self.is_machine_running = false,
            MachineStarted => {
                self.machine.reset(self.tape.clone());
                self.is_machine_running = true;
            }

            TableStatesNumberChanged(new_states_number) => {
                self.table.set_states_number(new_states_number)
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let (left_column, tasks_table, tape_preview) = if self.is_machine_running {
            self.get_running_machine_components()
        } else {
            self.get_not_running_machine_components()
        };

        let content = ui_column![tape_preview, row![left_column, tasks_table].spacing(40)]
            .spacing(20)
            .padding(40);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl App {
    fn get_running_machine_components(&self) -> (Column<Message>, Row<Message>, Column<Message>) {
        let left_column = {
            let stop_button = button("Stop")
                .padding(10)
                .width(Length::Fill)
                .on_press(Message::MachineStopped);

            let step = text(format!("Step: {}", self.machine.get_step()));
            let state = text(format!("State: {}", self.machine.get_state()));

            let next_step_button: Element<_> = if self.machine.is_halted() {
                text("Machine halted").into()
            } else {
                button("Next step")
                    .padding(10)
                    .width(Length::Fill)
                    .on_press(Message::MachineNextStep)
                    .into()
            };

            ui_column![step, state, next_step_button, stop_button]
                .width(Length::Units(200))
                .spacing(10)
        };

        let tasks_table = create_tasks_table(
            &self.table,
            &Message::TableTaskChanged,
            false,
            self.machine.get_tape().get_current_char(),
            self.machine.get_state(),
        );

        let tape_preview = create_tape_preview(&self.tape);

        (left_column, tasks_table, tape_preview)
    }

    fn get_not_running_machine_components(
        &self,
    ) -> (Column<Message>, Row<Message>, Column<Message>) {
        let left_column = {
            let initial_tape_input = text_input(
                "Set initial tape...",
                self.tape.get_chars_without_margin(),
                Message::TapeInputCharsChanged,
            )
            .padding(10)
            .size(20);

            let initial_cursor_position_input = number_input(
                "Set initial cursor position...",
                self.tape.get_cursor_position(),
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

            let start_button = button("Start")
                .padding(10)
                .width(Length::Fill)
                .on_press(Message::MachineStarted);

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
            ]
            .width(Length::Units(200))
            .spacing(10)
        };

        let tasks_table = create_tasks_table(
            &self.table,
            &Message::TableTaskChanged,
            true,
            self.tape.get_current_char(),
            DEFAULT_STATE,
        );

        let tape_preview = create_tape_preview(self.machine.get_tape());

        (left_column, tasks_table, tape_preview)
    }
}
