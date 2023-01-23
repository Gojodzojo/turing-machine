#![windows_subsystem = "windows"]
#![feature(iter_array_chunks)]

mod constants;
mod dialogs;
mod focus_actions;
mod language;
mod machine;
mod my_theme;
mod numeric_input;
mod scene;
mod table;
mod tape;
mod task;

use constants::{ICON_BYTES, ICON_FORMAT};
use dialogs::error_dialog;
use iced::theme::{self, Palette};
use iced::window::Icon;
use iced::{
    executor, keyboard, mouse, window, Application, Command, Element, Event, Settings,
    Subscription, Theme,
};
use iced_native::widget::Id;
use language::polish::POLISH_LANGUAGE;
use language::Language;
use machine::Machine;
use my_theme::MyTheme;
use scene::Scene;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;
use table::Table;
use tape::Tape;
use task::Task;

use crate::constants::SCALE_FACTOR_STEP;
use crate::dialogs::{pick_file_to_open_dialog, pick_file_to_save_dialog, unsaved_file_dialog};
use crate::focus_actions::{focus_next, get_focused_element_id};

pub fn main() -> iced::Result {
    App::run(Settings {
        exit_on_close_request: false,
        window: window::Settings {
            size: (1500, 800),
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
    focused_widget: Option<Id>,
    language: &'static Language,
    scale_factor: f64,
    is_side_column_opened: bool,
    palette: Palette,
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
    LanguageChanged(&'static Language),
    MachineSelfTimerIntervalChange(Option<u32>),
    ThemeChanged(MyTheme),
    OpenURL(&'static str),
    ToggleSideColumnClicked,
    NewFileClicked,
    OpenFileClicked,
    SaveFileClicked,
    SaveFileAsClicked,
    MachineStarted,
    MachineStopped,
    MachineNextStep,
    CloseButtonClicked,
    FocusedWidget(Option<Id>),
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
                focused_widget: None,
                language: POLISH_LANGUAGE,
                scale_factor: 1.0,
                is_side_column_opened: true,
                palette: Palette::LIGHT,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let filename = match &self.file_path {
            Some(path) => path.file_name().unwrap().to_str().unwrap(),
            None => self.language.default_filename,
        };

        let modified_indicator = match self.was_modified {
            true => "*",
            false => "",
        };

        format!(
            "{} - {}{}",
            self.language.app_name, filename, modified_indicator
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        use Message::*;

        match message {
            EventOccurred(e) => return self.handle_events(e),
            FocusedWidget(id) => self.focused_widget = id,
            OpenURL(url) => webbrowser::open(url).unwrap_or_else(|e| println!("{}", e)),
            CloseButtonClicked => self.should_exit = true,
            NewFileClicked => self.new_file(),
            ToggleSideColumnClicked => self.is_side_column_opened = !self.is_side_column_opened,
            ThemeChanged(theme) => self.palette = theme.palette,
            LanguageChanged(lang) => self.language = lang.into(),
            TapeInputCharsChanged(new_chars) => self.tape.set_chars(new_chars),
            TapeInputCursorPositionChanged(position) => self.tape.set_cursor_position(position),
            TapeLengthChanged(new_length) => self.tape.set_length(new_length),
            FileToOpenPicked(Some(path)) => return self.open_file(path),
            FileToSavePicked(Some(path)) => return self.save_file(path),
            OpenFileClicked => return pick_file_to_open_dialog(self.language),
            SaveFileAsClicked => return pick_file_to_save_dialog(self.language),
            SaveFileClicked => {
                return match &self.file_path {
                    Some(path) => self.save_file(path.clone()),
                    None => pick_file_to_save_dialog(self.language),
                }
            }
            WithUnsavedFileDialog(callback) => {
                return match self.was_modified {
                    true => unsaved_file_dialog(callback, self.language),
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
                            pick_file_to_save_dialog(self.language)
                        }
                    }
                }
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

    fn scale_factor(&self) -> f64 {
        self.scale_factor
    }

    fn theme(&self) -> Theme {
        theme::Theme::Custom(Box::new(theme::Custom::new(self.palette.clone())))
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
            return error_dialog(self.language.open_file_error_description, self.language);
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
            return error_dialog(self.language.save_file_error_description, self.language);
        }

        return Command::none();
    }

    fn handle_events(&mut self, e: Event) -> Command<Message> {
        use iced_native::Event::*;
        match e {
            Keyboard(keyboard::Event::KeyPressed {
                key_code,
                modifiers,
            }) => {
                use iced::keyboard::KeyCode::*;

                match key_code {
                    Plus | NumpadAdd if modifiers.control() => {
                        self.scale_factor += SCALE_FACTOR_STEP
                    }
                    Minus | NumpadSubtract if modifiers.control() => {
                        self.scale_factor -= SCALE_FACTOR_STEP
                    }
                    S if modifiers.control() => return redirect(Message::SaveFileClicked),
                    Tab => return focus_next(),
                    _ => {}
                }

                if self.scale_factor < SCALE_FACTOR_STEP {
                    self.scale_factor = SCALE_FACTOR_STEP;
                }
            }
            Window(window::Event::CloseRequested) => {
                return redirect(Message::WithUnsavedFileDialog(Box::new(
                    Message::CloseButtonClicked,
                )))
            }
            Mouse(mouse::Event::ButtonReleased(_))
            | Touch(iced::touch::Event::FingerLifted { id: _, position: _ }) => {
                return get_focused_element_id()
            }
            _ => {}
        }

        return Command::none();
    }
}

fn redirect(message: Message) -> Command<Message> {
    async fn noop() {}
    return Command::perform(noop(), |_| message);
}
