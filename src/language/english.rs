use crate::{
    constants::FILE_EXTENSION,
    dialogs::{AboutProgramDialogLabels, DialogTexsts},
    my_theme::ThemeNames,
    scene::{EditorSceneTexts, SideColumnTexts, SimulationSceneTexts},
};

use super::Language;

pub const ENGLISH_LANGUAGE: &Language = &Language {
    language_name: "English",
    app_name: "Turing Machine",
    default_filename: const_str::concat!("new.", FILE_EXTENSION),
    dialog_texts: DialogTexsts {
        file_filter_name: "Turing Machine file",
        open_file_error_description: "Wrong file format",
        save_file_error_description: "Failed to save the file",
        error_message_title: "Error",
        unsaved_file_dialog_title: "Unsaved changes",
        unsaved_file_dialog_description:
            "This file contains unsaved changes. Do you want to save this file?",
    },
    editor_scene_texts: EditorSceneTexts {
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
    },
    side_column_texts: SideColumnTexts {
        new_file_button_text: "New file",
        open_file_button_text: "Open file",
        save_file_button_text: "Save file",
        save_file_as_button_text: "Save file as",
        help_button_text: "Help",
        help_url: "https://github.com/Gojodzojo/turing-machine/blob/main/readmes/README_EN.md",
        about_program_button_text: "About",
        theme_picker_label: "Theme",
    },
    simulation_scene_texts: SimulationSceneTexts {
        machine_halted_text: "Machine halted",
        self_timer_interval_none_text: "None",
        self_timer_interval_input_label: "Self-timer interval",
        stop_machine_button_text: "Stop",
        next_step_button_text: "Next step",
        step_text: "Step",
        state_text: "State",
    },
    theme_names: ThemeNames {
        dark: "Dark",
        light: "Light",
        tokyo_night: "Tokyo Night",
    },
    about_program_dialog_labels: AboutProgramDialogLabels {
        program_name_label: "Program name",
        author_label: "Author",
        program_version_label: "Program Version",
        repository_label: "Repository",
    },
};
