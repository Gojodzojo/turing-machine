use crate::{
    constants::FILE_EXTENSION,
    dialogs::{AboutProgramDialogLabels, DialogTexsts},
    my_theme::ThemeNames,
    scene::{EditorSceneTexts, SideColumnTexts, SimulationSceneTexts},
};

use super::Language;

pub const POLISH_LANGUAGE: &Language = &Language {
    language_name: "Polski",
    app_name: "Maszyna Turinga",
    default_filename: const_str::concat!("nowy.", FILE_EXTENSION),
    dialog_texts: DialogTexsts {
        file_filter_name: "Plik Maszyny Turinga",
        open_file_error_description: "Zły format pliku",
        save_file_error_description: "Nie udało się zapisać pliku",
        error_message_title: "Błąd",
        unsaved_file_dialog_title: "Niezapisane zmiany",
        unsaved_file_dialog_description:
            "Ten plik zawiera niezapisane zmiany. Czy chcesz je zapisać?",
    },
    editor_scene_texts: EditorSceneTexts {
        initial_tape_input_placeholder: "Wpisz tekst taśmy...",
        initial_tape_input_label: "Tekst taśmy",
        initial_cursor_position_input_placeholder: "Ustaw pozycję kursora...",
        initial_cursor_position_input_label: "Pozycja kursora",
        tape_length_input_placeholder: "Ustaw długość taśmy...",
        tape_length_input_label: "Długość taśmy",
        table_characters_input_placeholder: "Wpisz znaki tablicy...",
        table_characters_input_label: "Znaki tablicy",
        table_states_number_input_placeholder: "Ustaw liczbę stanów tablicy...",
        table_states_number_input_label: "Liczba stanów tablicy",
        start_machine_button_text: "Start",
    },
    side_column_texts: SideColumnTexts {
        new_file_button_text: "Nowy plik",
        open_file_button_text: "Otwórz plik",
        save_file_button_text: "Zapisz plik",
        save_file_as_button_text: "Zapisz plik jako",
        help_button_text: "Pomoc",
        help_url: "https://github.com/Gojodzojo/turing-machine/blob/main/readmes/README_PL.md",
        about_program_button_text: "O programie",
        theme_picker_label: "Motyw",
    },
    simulation_scene_texts: SimulationSceneTexts {
        machine_halted_text: "Maszyna zatrzymana",
        self_timer_interval_none_text: "Brak",
        self_timer_interval_input_label: "Interwał samowyzwalacza",
        stop_machine_button_text: "Stop",
        next_step_button_text: "Następny krok",
        step_text: "Krok",
        state_text: "Stan",
    },
    theme_names: ThemeNames {
        dark: "Ciemny",
        light: "Jasny",
        tokyo_night: "Tokyo Night",
    },
    about_program_dialog_labels: AboutProgramDialogLabels {
        program_name_label: "Nazwa programu",
        author_label: "Autor",
        program_version_label: "Wersja programu",
        repository_label: "Repozytorium",
    },
};
