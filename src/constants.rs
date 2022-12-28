use iced::Font;

use crate::task::Direction;

pub const EMPTY_CHAR: char = '#';
pub const DEFAULT_STATE: usize = 0;
pub const DEFAULT_TASK_CHAR: char = '0';
pub const DEFAULT_TASK_DIRECTION: Direction = Direction::Stop;
pub const MAX_TAPE_CHARS_NUMBER: usize = 101;
pub const TAPE_FONT: Font = Font::External {
    name: "Roboto Mono",
    bytes: include_bytes!("../RobotoMono/fonts/ttf/RobotoMono-Medium.ttf"),
};
