use iced::Theme;

use crate::language::Language;

impl std::fmt::Display for MyTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.theme_name)
    }
}

#[derive(Debug, Clone)]
pub struct MyTheme {
    theme_name: &'static str,
    pub theme: Theme,
}

impl PartialEq for MyTheme {
    fn eq(&self, other: &Self) -> bool {
        self.theme == other.theme
    }
}

impl Eq for MyTheme {
    fn assert_receiver_is_total_eq(&self) {}
}

impl MyTheme {
    pub fn from_theme(theme: Theme, language: &'static Language) -> Self {
        let theme_name = match theme {
            Theme::Light => language.theme_names.light,
            Theme::Dark => language.theme_names.dark,
            _ => unreachable!(),
        };

        MyTheme { theme_name, theme }
    }

    pub fn all(language: &'static Language) -> [MyTheme; 2] {
        [
            Self::from_theme(Theme::Light, language),
            Self::from_theme(Theme::Dark, language),
        ]
    }
}

#[derive(PartialEq, Eq)]
pub struct ThemeNames {
    pub light: &'static str,
    pub dark: &'static str,
}
