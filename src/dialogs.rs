use std::path::PathBuf;

use iced::Command;
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageLevel};

use crate::{constants::FILE_EXTENSION, language::Language, Message};

#[derive(PartialEq, Eq)]
pub struct DialogTexsts {
    pub open_file_error_description: &'static str,
    pub save_file_error_description: &'static str,
    pub error_message_title: &'static str,
    pub unsaved_file_dialog_title: &'static str,
    pub unsaved_file_dialog_description: &'static str,
    pub file_filter_name: &'static str,
}

pub fn error_dialog(description: &'static str, language: &'static Language) -> Command<Message> {
    async fn a(description: &str, language: &'static Language) {
        MessageDialog::new()
            .set_level(MessageLevel::Error)
            .set_title(language.dialog_texts.error_message_title)
            .set_description(description)
            .set_buttons(MessageButtons::Ok)
            .show();
    }
    return Command::perform(a(description, language), Message::ErrorDialogClosed);
}

pub fn unsaved_file_dialog(
    callback: Box<Message>,
    language: &'static Language,
) -> Command<Message> {
    async fn a(callback: Box<Message>, language: &'static Language) -> (bool, Box<Message>) {
        let choice = MessageDialog::new()
            .set_level(MessageLevel::Info)
            .set_title(language.dialog_texts.unsaved_file_dialog_title)
            .set_description(language.dialog_texts.unsaved_file_dialog_description)
            .set_buttons(rfd::MessageButtons::YesNo)
            .show();

        (choice, callback)
    }

    return Command::perform(a(callback, language), Message::UnsavedFileDialogAnsweared);
}

pub fn pick_file_to_open_dialog(language: &'static Language) -> Command<Message> {
    async fn a(language: &'static Language) -> Option<PathBuf> {
        FileDialog::new()
            .add_filter(language.dialog_texts.file_filter_name, &[FILE_EXTENSION])
            .pick_file()
    }

    return Command::perform(a(language), Message::FileToOpenPicked);
}

pub fn pick_file_to_save_dialog(language: &'static Language) -> Command<Message> {
    async fn a(language: &'static Language) -> Option<PathBuf> {
        let path = FileDialog::new()
            .add_filter(language.dialog_texts.file_filter_name, &[FILE_EXTENSION])
            .set_file_name(language.default_filename)
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

    return Command::perform(a(language), Message::FileToSavePicked);
}

#[derive(PartialEq, Eq)]
pub struct AboutProgramDialogLabels {
    pub program_name_label: &'static str,
    pub author_label: &'static str,
    pub program_version_label: &'static str,
    pub repository_label: &'static str,
}

pub fn about_program_dialog(language: &'static Language) -> Command<Message> {
    async fn a(language: &'static Language) {
        let AboutProgramDialogLabels {
            program_name_label,
            author_label,
            program_version_label,
            repository_label,
        } = language.about_program_dialog_labels;

        let text = format!(
            "{}: Turing Machine
{}: Mateusz Goik
{}: {}
{}: https://github.com/Gojodzojo/turing-machine",
            program_name_label,
            author_label,
            program_version_label,
            option_env!("CARGO_PKG_VERSION").unwrap(),
            repository_label
        );

        MessageDialog::new()
            .set_level(MessageLevel::Info)
            .set_title(language.side_column_texts.about_program_button_text)
            .set_description(&text)
            .set_buttons(MessageButtons::Ok)
            .show();
    }
    return Command::perform(a(language), Message::ErrorDialogClosed);
}
