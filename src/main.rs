#![feature(iter_array_chunks)]

mod constants;
mod machine;
mod number_input;
mod scene;
mod table;
mod tape;
mod task;

use constants::{DEFAULT_FILENAME, FILE_EXTENSION};
use iced::{executor, Application, Command, Element, Settings, Theme};
use machine::Machine;
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageLevel};
use scene::Scene;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;
use table::Table;
use tape::Tape;
use task::Task;

pub fn main() -> iced::Result {
    App::run(Settings::default())
}

pub struct App {
    table: Table,
    machine: Machine,
    tape: Tape,
    file_path: Option<PathBuf>,
    was_modified: bool,
    scene: Scene,
}

#[derive(Debug, Clone)]
pub enum Message {
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
                table: Table::new_empty(),
                machine: Machine::new(),
                tape: Tape::new(),
                file_path: None,
                was_modified: true,
                scene: Scene::Editor,
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
            MachineStopped => self.scene = Scene::Editor,
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
                self.scene = Scene::Machine;
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
        self.scene.view(self)
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
