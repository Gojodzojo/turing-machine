#![feature(iter_array_chunks)]

mod constants;
mod machine;
mod number_input;
mod table;
mod tape;
mod task;

use constants::{DEFAULT_FILENAME, DEFAULT_STATE, FILE_EXTENSION};
use iced::widget::{button, column as ui_column, container, row, text, text_input, Column, Row};
use iced::{executor, Application, Command, Element, Length, Settings, Theme};
use machine::Machine;
use number_input::number_input;
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageLevel};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;
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
    file_path: Option<PathBuf>,
    was_modified: bool,
}

#[derive(Debug, Clone)]
enum Message {
    TapeInputCharsChanged(String),
    TapeInputCursorPositionChanged(isize),
    TableCharactersChanged(String),
    TableStatesNumberChanged(usize),
    TableTaskChanged(Task, usize, usize),
    FileToOpenPicked(Option<PathBuf>),
    FileToSavePicked(Option<PathBuf>),
    NewFileClicked,
    OpenFileClicked,
    SaveFileClicked,
    SaveFileAsClicked,
    MachineStarted,
    MachineStopped,
    MachineNextStep,
    ErrorDialogClosed(()),
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                is_machine_running: false,
                table: Table::new_empty(),
                machine: Machine::new(),
                tape: Tape::new(),
                file_path: None,
                was_modified: true,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let filename = match &self.file_path {
            Some(path) => path.file_name().unwrap().to_str().unwrap(),
            None => DEFAULT_FILENAME,
        };

        let modified_indicator = match self.was_modified {
            true => "*",
            false => "",
        };

        format!("Turing Machine - {}{}", filename, modified_indicator)
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        use Message::*;

        match message {
            FileToOpenPicked(Some(path)) => return self.open_file(path),
            FileToSavePicked(Some(path)) => return self.save_file(path),
            TapeInputCharsChanged(new_chars) => self.tape.set_chars(new_chars),
            TapeInputCursorPositionChanged(position) => self.tape.set_cursor_position(position),
            MachineNextStep => self.machine.next_step(&self.table),
            MachineStopped => self.is_machine_running = false,
            NewFileClicked => self.new_file(),
            OpenFileClicked => return Command::perform(pick_file_to_open(), FileToOpenPicked),
            SaveFileClicked => {
                return match &self.file_path {
                    Some(path) => self.save_file(path.clone()),
                    None => Command::perform(pick_file_to_save(DEFAULT_FILENAME), FileToSavePicked),
                }
            }
            SaveFileAsClicked => {
                return Command::perform(pick_file_to_save(DEFAULT_FILENAME), FileToSavePicked)
            }
            MachineStarted => {
                self.machine.reset(self.tape.clone());
                self.is_machine_running = true;
            }
            TableCharactersChanged(new_characters) => {
                self.table.set_characters(&new_characters);
                self.was_modified = true;
            }
            TableTaskChanged(task, row, column) => {
                self.table.set_task_by_position(task, row, column);
                self.was_modified = true;
            }
            TableStatesNumberChanged(new_states_number) => {
                self.table.set_states_number(new_states_number);
                self.was_modified = true;
            }
            _ => {}
        };

        return Command::none();
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
    fn new_file(&mut self) {
        self.table = Table::new_empty();
        self.was_modified = true;
        self.file_path = None;
    }

    fn open_file(&mut self, path: PathBuf) -> Command<Message> {
        let res = || -> Result<(), io::Error> {
            let file = File::open(&path)?;
            let mut buffer = BufReader::new(file);
            self.table = Table::new_from_buffer(&mut buffer)?;
            self.was_modified = false;
            self.file_path = Some(path);
            Ok(())
        };

        if let Err(_) = res() {
            return Command::perform(
                show_error("Error", "Wrong file format"),
                Message::ErrorDialogClosed,
            );
        }

        return Command::none();
    }

    fn save_file(&mut self, path: PathBuf) -> Command<Message> {
        let res = || -> Result<(), io::Error> {
            let mut file = File::create(&path)?;
            self.table.write_to_buffer(&mut file)?;
            self.was_modified = false;
            self.file_path = Some(path);
            Ok(())
        };

        if let Err(_) = res() {
            return Command::perform(
                show_error("Error", "Failed to save the file"),
                Message::ErrorDialogClosed,
            );
        }

        return Command::none();
    }

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

        let tape_preview = create_tape_preview(self.machine.get_tape());

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
        };

        let tasks_table = create_tasks_table(
            &self.table,
            &Message::TableTaskChanged,
            true,
            self.tape.get_current_char(),
            DEFAULT_STATE,
        );

        let tape_preview = create_tape_preview(&self.tape);

        (left_column, tasks_table, tape_preview)
    }
}

async fn show_error(title: &str, description: &str) {
    MessageDialog::new()
        .set_level(MessageLevel::Error)
        .set_title(title)
        .set_description(description)
        .set_buttons(MessageButtons::Ok)
        .show();
}

async fn pick_file_to_open() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Turing Machine file", &[FILE_EXTENSION])
        .pick_file()
}

async fn pick_file_to_save(default_filename: &str) -> Option<PathBuf> {
    let path = FileDialog::new()
        .add_filter("Turing Machine file", &[FILE_EXTENSION])
        .set_file_name(default_filename)
        .save_file();

    if let Some(mut path) = path {
        match path.extension() {
            Some(ext) if ext == FILE_EXTENSION => {}
            _ => {
                let new_filename = format!("{}.{}", path.file_name()?.to_str()?, FILE_EXTENSION);
                path.set_file_name(new_filename);
            }
        }

        return Some(path);
    }

    return None;
}
