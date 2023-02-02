use std::str::FromStr;

use crate::{
    dialogs::{AboutProgramDialogLabels, DialogTexsts},
    my_theme::ThemeNames,
    scene::{EditorSceneTexts, SideColumnTexts, SimulationSceneTexts},
};

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

impl FromStr for &Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for language in ALL_LANGUAGES {
            if s == language.language_name {
                return Ok(language);
            }
        }

        Err(())
    }
}

#[derive(Eq)]
pub struct Language {
    pub language_name: &'static str,
    pub app_name: &'static str,
    pub default_filename: &'static str,
    pub dialog_texts: DialogTexsts,
    pub editor_scene_texts: EditorSceneTexts,
    pub side_column_texts: SideColumnTexts,
    pub simulation_scene_texts: SimulationSceneTexts,
    pub theme_names: ThemeNames,
    pub about_program_dialog_labels: AboutProgramDialogLabels,
}
