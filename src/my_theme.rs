use std::str::FromStr;

use iced::{theme::Palette, Color};

use crate::language::{english::ENGLISH_LANGUAGE, Language};

impl std::fmt::Display for MyTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.theme_name)
    }
}

const TOKYO_NIGHT_PALETTE: Palette = Palette {
    background: Color::from_rgb(32 as f32 / 255.0, 36 as f32 / 255.0, 52 as f32 / 255.0),
    text: Color::from_rgb(169 as f32 / 255.0, 177 as f32 / 255.0, 214 as f32 / 255.0),
    primary: Color::from_rgb(122 as f32 / 255.0, 162 as f32 / 255.0, 247 as f32 / 255.0),
    success: Color::from_rgb(158 as f32 / 255.0, 206 as f32 / 255.0, 206 as f32 / 255.0),
    danger: Color::from_rgb(247 as f32 / 255.0, 118 as f32 / 255.0, 142 as f32 / 255.0),
};

#[derive(Debug, Clone)]
pub struct MyTheme {
    pub theme_name: &'static str,
    pub palette: Palette,
}

impl PartialEq for MyTheme {
    fn eq(&self, other: &Self) -> bool {
        self.palette == other.palette
    }
}

impl Eq for MyTheme {
    fn assert_receiver_is_total_eq(&self) {}
}

impl FromStr for MyTheme {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for theme in Self::all(ENGLISH_LANGUAGE) {
            if s == theme.theme_name {
                return Ok(theme);
            }
        }

        Err(())
    }
}

impl MyTheme {
    pub fn from_palette(palette: Palette, language: &'static Language) -> Self {
        let theme_name = match palette {
            p if are_palettes_equal(p, Palette::LIGHT) => language.theme_names.light,
            p if are_palettes_equal(p, Palette::DARK) => language.theme_names.dark,
            p if are_palettes_equal(p, TOKYO_NIGHT_PALETTE) => language.theme_names.tokyo_night,
            _ => unreachable!(),
        };

        MyTheme {
            theme_name,
            palette,
        }
    }

    pub fn all(language: &'static Language) -> [MyTheme; 3] {
        [
            Self::from_palette(Palette::LIGHT, language),
            Self::from_palette(Palette::DARK, language),
            Self::from_palette(TOKYO_NIGHT_PALETTE, language),
        ]
    }
}

fn are_palettes_equal(a: Palette, b: Palette) -> bool {
    a.background == b.background
        && a.text == b.text
        && a.primary == b.primary
        && a.success == b.success
        && a.danger == b.danger
}

#[derive(PartialEq, Eq)]
pub struct ThemeNames {
    pub light: &'static str,
    pub dark: &'static str,
    pub tokyo_night: &'static str,
}
