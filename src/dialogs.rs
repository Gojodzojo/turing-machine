use std::path::PathBuf;

use iced::Command;
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageLevel};

use crate::{constants::FILE_EXTENSION, language::Language, Message};

pub fn error_dialog(description: &'static str, language: &'static Language) -> Command<Message> {
    async fn a(description: &str, language: &'static Language) {
        MessageDialog::new()
            .set_level(MessageLevel::Error)
            .set_title(language.error_message_title)
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
            .set_title(language.unsaved_file_dialog_title)
            .set_description(language.unsaved_file_dialog_description)
            .set_buttons(rfd::MessageButtons::YesNo)
            .show();

        (choice, callback)
    }

    return Command::perform(a(callback, language), Message::UnsavedFileDialogAnsweared);
}

pub fn pick_file_to_open_dialog(language: &'static Language) -> Command<Message> {
    async fn a(language: &'static Language) -> Option<PathBuf> {
        FileDialog::new()
            .add_filter(language.file_filter_name, &[FILE_EXTENSION])
            .pick_file()
    }

    return Command::perform(a(language), Message::FileToOpenPicked);
}

pub fn pick_file_to_save_dialog(language: &'static Language) -> Command<Message> {
    async fn a(language: &'static Language) -> Option<PathBuf> {
        let path = FileDialog::new()
            .add_filter(language.file_filter_name, &[FILE_EXTENSION])
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