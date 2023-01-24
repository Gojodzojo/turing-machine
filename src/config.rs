use std::{
    fs::{create_dir_all, File},
    io::{self, prelude::*, BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use dirs::home_dir;
use iced::theme::Palette;

use crate::{
    language::{english::ENGLISH_LANGUAGE, Language},
    my_theme::MyTheme,
    App,
};

pub fn load_config() -> io::Result<(&'static Language, Palette)> {
    let invalid_data_error = || io::Error::from(io::ErrorKind::InvalidData);
    let path = get_config_path();
    let file = File::open(path)?;

    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines();

    let line = lines_iter.next().ok_or(invalid_data_error())??;
    let language: &Language = FromStr::from_str(&line).or(Err(invalid_data_error()))?;

    let line = lines_iter
        .next()
        .ok_or(io::Error::from(invalid_data_error()))??;
    let theme: MyTheme = FromStr::from_str(&line).or(Err(invalid_data_error()))?;

    Ok((language, theme.palette))
}

pub fn save_config(app: &App) -> io::Result<()> {
    let path = get_config_path();
    create_dir_all(path.parent().unwrap())?;
    let mut file = File::create(path)?;

    writeln!(file, "{}", app.language.language_name)?;
    writeln!(
        file,
        "{}",
        MyTheme::from_palette(app.palette, ENGLISH_LANGUAGE).theme_name
    )?;

    Ok(())
}

fn get_config_path() -> PathBuf {
    #[cfg(target_family = "windows")]
    return PathBuf::from("C:\\Program Files (x86)\\turing-machine\\config.txt");

    #[cfg(target_family = "unix")]
    return home_dir()
        .unwrap()
        .join(".turing-machine")
        .join("config.txt");
}
