use crate::task::Direction;
use iced::Font;
use image::ImageFormat;

pub const FILE_EXTENSION: &str = "mt";
pub const DEFAULT_FILENAME: &str = const_str::concat!("new.", FILE_EXTENSION);
pub const EMPTY_CHAR: char = '#';
pub const DEFAULT_TABLE_CHARS: &str = const_str::concat!(EMPTY_CHAR, "abc");
pub const DEFAULT_STATE: usize = 0;
pub const DEFAULT_TASK_CHAR: char = '0';
pub const DEFAULT_TASK_DIRECTION: Direction = Direction::Stop;
pub const DEFAULT_TAPE_CHARS_NUMBER: usize = 101;
pub const MAX_TAPE_LENGTH: usize = 250;
pub const MIN_TAPE_LENGTH: usize = 1;
pub const MAX_STATES_NUMBER: usize = 16;
pub const MIN_STATES_NUMBER: usize = 1;
pub const MAX_TAPE_FONT_SIZE: u16 = 30;
pub const MIN_TAPE_FONT_SIZE: u16 = 14;
pub const MACHINE_SELF_TIMER_INTERVAL_STEP: u32 = 100;
pub const MIN_MACHINE_SELF_TIMER_INTERVAL: u32 = MACHINE_SELF_TIMER_INTERVAL_STEP * 1;
pub const MAX_MACHINE_SELF_TIMER_INTERVAL: u32 = MACHINE_SELF_TIMER_INTERVAL_STEP * 10;
pub const STOP_MACHINE_SELF_TIMER_VALUE: u32 =
    MAX_MACHINE_SELF_TIMER_INTERVAL + MACHINE_SELF_TIMER_INTERVAL_STEP;
pub const TAPE_FONT: Font = Font::External {
    name: "Roboto Mono",
    bytes: include_bytes!("../RobotoMono/fonts/ttf/RobotoMono-Medium.ttf"),
};
pub const ICON_BYTES: &[u8] = include_bytes!("../icon.ico");
pub const ICON_FORMAT: ImageFormat = ImageFormat::Ico;
