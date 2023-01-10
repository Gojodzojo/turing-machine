use crate::task::Direction;
use iced::Font;

pub const FILE_EXTENSION: &str = "mt";
pub const DEFAULT_FILENAME: &str = const_str::concat!("new.", FILE_EXTENSION);
pub const EMPTY_CHAR: char = '#';
pub const DEFAULT_TABLE_CHARS: &str = const_str::concat!(EMPTY_CHAR, "abc");
pub const DEFAULT_STATE: usize = 0;
pub const DEFAULT_TASK_CHAR: char = '0';
pub const DEFAULT_TASK_DIRECTION: Direction = Direction::Stop;
pub const DEFAULT_TAPE_CHARS_NUMBER: usize = 101;
pub const MAX_TAPE_LENGTH: usize = 250;
pub const MIN_TAPE_LENGTH: usize = 3;
pub const MAX_STATES_NUMBER: usize = 16;
pub const MIN_STATES_NUMBER: usize = 1;
pub const MAX_TAPE_FONT_SIZE: u16 = 30;
pub const MIN_TAPE_FONT_SIZE: u16 = 14;
pub const TAPE_FONT: Font = Font::External {
    name: "Roboto Mono",
    bytes: include_bytes!("../RobotoMono/fonts/ttf/RobotoMono-Medium.ttf"),
};
