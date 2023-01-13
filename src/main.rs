#![windows_subsystem = "windows"]
#![feature(iter_array_chunks)]

mod constants;
mod machine;
mod numeric_input;
mod scene;
mod table;
mod tape;
mod task;

use constants::{DEFAULT_FILENAME, FILE_EXTENSION, ICON_BYTES, ICON_FORMAT};
use iced::window::Icon;
use iced::{executor, window, Application, Command, Element, Settings, Subscription, Theme};
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
    App::run(Settings {
        exit_on_close_request: false,
        window: window::Settings {
            icon: Icon::from_file_data(ICON_BYTES, Some(ICON_FORMAT)).ok(),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

pub struct App {
    table: Table,
    machine: Machine,
    tape: Tape,
    file_path: Option<PathBuf>,
    was_modified: bool,
    scene: Scene,
    should_exit: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    TapeInputCharsChanged(String),
    TapeInputCursorPositionChanged(isize),
    TapeLengthChanged(usize),
    TableCharactersChanged(String),
    TableStatesNumberChanged(usize),
    TableTaskChanged(Task, usize, usize),
    FileToOpenPicked(Option<PathBuf>),
    FileToSavePicked(Option<PathBuf>),
    MachineSelfTimerIntervalChange(Option<u32>),
    NewFileClicked,
    OpenFileClicked,
    SaveFileClicked,
    SaveFileAsClicked,
    MachineStarted,
    MachineStopped,
    MachineNextStep,
    CloseButtonClicked,
    ErrorDialogClosed(()),
    EventOccurred(iced_native::Event),
    WithUnsavedFileDialog(Box<Message>),
    UnsavedFileDialogAnsweared((bool, Box<Message>)),
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
                was_modified: false,
                scene: Scene::Editor,
                should_exit: false,
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
            CloseButtonClicked => self.should_exit = true,
            NewFileClicked => self.new_file(),
            TapeInputCharsChanged(new_chars) => self.tape.set_chars(new_chars),
            TapeInputCursorPositionChanged(position) => self.tape.set_cursor_position(position),
            TapeLengthChanged(new_length) => self.tape.set_length(new_length),
            FileToOpenPicked(Some(path)) => return self.open_file(path),
            FileToSavePicked(Some(path)) => return self.save_file(path),
            OpenFileClicked => return pick_file_to_open(),
            SaveFileAsClicked => return pick_file_to_save(),
            SaveFileClicked => {
                return match &self.file_path {
                    Some(path) => self.save_file(path.clone()),
                    None => pick_file_to_save(),
                }
            }
            WithUnsavedFileDialog(callback) => {
                return match self.was_modified {
                    true => show_should_save_dialog(callback),
                    false => redirect(*callback),
                }
            }
            UnsavedFileDialogAnsweared((choice, callback)) => {
                return match choice {
                    false => redirect(*callback),
                    true => {
                        if let Some(path) = &self.file_path {
                            Command::batch([self.save_file(path.clone()), redirect(*callback)])
                        } else {
                            pick_file_to_save()
                        }
                    }
                }
            }
            EventOccurred(iced_native::Event::Window(window::Event::CloseRequested)) => {
                return redirect(WithUnsavedFileDialog(Box::new(CloseButtonClicked)));
            }
            MachineNextStep => self.machine.next_step(&self.table),
            MachineStarted => {
                self.machine.reset(self.tape.clone());
                self.scene = Scene::Machine;
            }
            MachineStopped => {
                self.machine.set_self_timer_interval(None);
                self.scene = Scene::Editor
            }
            MachineSelfTimerIntervalChange(interval) => {
                self.machine.set_self_timer_interval(interval)
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

    fn subscription(&self) -> Subscription<Message> {
        let mut subscriptions = Vec::with_capacity(2);
        subscriptions.push(iced::subscription::events().map(Message::EventOccurred));
        if let Some(interval) = self.machine.get_self_timer_interval() {
            subscriptions.push(iced::time::every(interval).map(|_| Message::MachineNextStep))
        }

        Subscription::batch(subscriptions)
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}

impl App {
    fn new_file(&mut self) {
        self.table = Table::new_empty();
        self.was_modified = false;
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
            return show_error("Wrong file format");
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
            return show_error("Failed to save the file");
        }

        return Command::none();
    }
}

fn show_error(description: &'static str) -> Command<Message> {
    async fn a(description: &str) {
        MessageDialog::new()
            .set_level(MessageLevel::Error)
            .set_title("Error")
            .set_description(description)
            .set_buttons(MessageButtons::Ok)
            .show();
    }
    return Command::perform(a(description), Message::ErrorDialogClosed);
}

fn show_should_save_dialog(callback: Box<Message>) -> Command<Message> {
    async fn a(callback: Box<Message>) -> (bool, Box<Message>) {
        let choice = MessageDialog::new()
            .set_level(MessageLevel::Info)
            .set_title("Unsaved changes")
            .set_description("This file contains unsaved changes. Do you want to save this file?")
            .set_buttons(rfd::MessageButtons::YesNo)
            .show();

        (choice, callback)
    }

    return Command::perform(a(callback), Message::UnsavedFileDialogAnsweared);
}

fn pick_file_to_open() -> Command<Message> {
    async fn a() -> Option<PathBuf> {
        FileDialog::new()
            .add_filter("Turing Machine file", &[FILE_EXTENSION])
            .pick_file()
    }

    return Command::perform(a(), Message::FileToOpenPicked);
}

fn pick_file_to_save() -> Command<Message> {
    async fn a() -> Option<PathBuf> {
        let path = FileDialog::new()
            .add_filter("Turing Machine file", &[FILE_EXTENSION])
            .set_file_name(DEFAULT_FILENAME)
            .save_file();

        if let Some(mut path) = path {
            match path.extension() {
                Some(ext) if ext == FILE_EXTENSION => {}
                _ => {
                    let new_filename =
                        format!("{}.{}", path.file_name()?.to_str()?, FILE_EXTENSION);
                    path.set_file_name(new_filename);
                }
            }

            return Some(path);
        }

        return None;
    }

    return Command::perform(a(), Message::FileToSavePicked);
}

fn redirect(message: Message) -> Command<Message> {
    async fn noop() {}
    return Command::perform(noop(), |_| message);
}
