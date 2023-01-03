use crate::task::Direction;
use iced::Font;

pub const EMPTY_CHAR: char = '#';
pub const DEFAULT_TABLE_CHARS: &str = const_str::concat!(EMPTY_CHAR, "abc");
pub const DEFAULT_STATE: usize = 0;
pub const DEFAULT_TASK_CHAR: char = '0';
pub const DEFAULT_TASK_DIRECTION: Direction = Direction::Stop;
pub const TAPE_CHARS_NUMBER: usize = 101;
pub const MAX_STATES_NUMBER: usize = 16;
pub const MIN_STATES_NUMBER: usize = 1;
pub const TAPE_TEXT_SIZE: u16 = 25;
pub const TAPE_FONT: Font = Font::External {
    name: "Roboto Mono",
    bytes: include_bytes!("../RobotoMono/fonts/ttf/RobotoMono-Medium.ttf"),
};
