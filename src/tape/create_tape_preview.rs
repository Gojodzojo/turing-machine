use std::iter::repeat;

use iced::widget::{column as ui_column, text, Column};

use crate::constants::{MAX_TAPE_CHARS_NUMBER, TAPE_FONT};

use super::Tape;

const SPACES: &str = const_str::repeat!(" ", MAX_TAPE_CHARS_NUMBER);

pub fn create_tape_preview<'a, Message: 'a + Clone>(tape: &Tape) -> Column<'a, Message> {
    let margin_left = (tape.get_position_zero() + tape.get_tape_cursor_position()) as usize;
    ui_column![
        text(tape.get_tape_chars()).font(TAPE_FONT),
        text(format!("{}^", &SPACES[0..margin_left])).font(TAPE_FONT),
    ]
}
