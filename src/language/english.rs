use crate::{constants::FILE_EXTENSION, my_theme::ThemeNames};

use super::Language;

pub const ENGLISH_LANGUAGE: &Language = &Language {
    language_name: "English",
    app_name: "Turing Machine",
    open_file_error_description: "Wrong file format",
    save_file_error_description: "Failed to save the file",
    error_message_title: "Error",
    unsaved_file_dialog_title: "Unsaved changes",
    unsaved_file_dialog_description:
        "This file contains unsaved changes. Do you want to save this file?",
    file_filter_name: "Turing Machine file",
    default_filename: const_str::concat!("new.", FILE_EXTENSION),
    initial_tape_input_placeholder: "Set initial tape...",
    initial_tape_input_label: "Tape text",
    initial_cursor_position_input_placeholder: "Set initial cursor position...",
    initial_cursor_position_input_label: "Cursor position",
    tape_length_input_placeholder: "Set tape length...",
    tape_length_input_label: "Tape length",
    table_characters_input_placeholder: "Set table characters...",
    table_characters_input_label: "Table characters",
    table_states_number_input_placeholder: "Set table states number...",
    table_states_number_input_label: "Table states number",
    start_machine_button_text: "Start",
    new_file_button_text: "New file",
    open_file_button_text: "Open file",
    save_file_button_text: "Save file",
    save_file_as_button_text: "Save file as",
    machine_halted_text: "Machine halted",
    self_timer_interval_none_text: "None",
    self_timer_interval_input_label: "Self-timer interval",
    stop_machine_button_text: "Stop",
    next_step_button_text: "Next step",
    step_text: "Step",
    state_text: "State",
    theme_picker_label: "Theme",
    theme_names: ThemeNames {
        dark: "Dark",
        light: "Light",
    },
};
