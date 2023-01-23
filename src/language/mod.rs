use crate::my_theme::ThemeNames;

pub mod english;
pub mod polish;

pub const ALL_LANGUAGES: [&Language; 2] = [polish::POLISH_LANGUAGE, english::ENGLISH_LANGUAGE];

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.language_name)
    }
}

impl std::fmt::Debug for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.language_name)
    }
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.language_name == other.language_name
    }
}

#[derive(Eq)]
pub struct Language {
    pub language_name: &'static str,
    pub app_name: &'static str,
    pub open_file_error_description: &'static str,
    pub save_file_error_description: &'static str,
    pub error_message_title: &'static str,
    pub unsaved_file_dialog_title: &'static str,
    pub unsaved_file_dialog_description: &'static str,
    pub file_filter_name: &'static str,
    pub default_filename: &'static str,
    pub initial_tape_input_placeholder: &'static str,
    pub initial_tape_input_label: &'static str,
    pub initial_cursor_position_input_placeholder: &'static str,
    pub initial_cursor_position_input_label: &'static str,
    pub tape_length_input_placeholder: &'static str,
    pub tape_length_input_label: &'static str,
    pub table_characters_input_placeholder: &'static str,
    pub table_characters_input_label: &'static str,
    pub table_states_number_input_placeholder: &'static str,
    pub table_states_number_input_label: &'static str,
    pub start_machine_button_text: &'static str,
    pub new_file_button_text: &'static str,
    pub open_file_button_text: &'static str,
    pub save_file_button_text: &'static str,
    pub save_file_as_button_text: &'static str,
    pub help_button_text: &'static str,
    pub help_url: &'static str,
    pub machine_halted_text: &'static str,
    pub self_timer_interval_none_text: &'static str,
    pub self_timer_interval_input_label: &'static str,
    pub stop_machine_button_text: &'static str,
    pub next_step_button_text: &'static str,
    pub step_text: &'static str,
    pub state_text: &'static str,
    pub theme_picker_label: &'static str,
    pub theme_names: ThemeNames,
}
