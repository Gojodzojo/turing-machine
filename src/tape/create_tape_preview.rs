use iced::widget::{column as ui_column, text, Column};

use crate::constants::{TAPE_CHARS_NUMBER, TAPE_FONT, TAPE_TEXT_SIZE};

use super::Tape;

const SPACES: &str = const_str::repeat!(" ", TAPE_CHARS_NUMBER);

pub fn create_tape_preview<'a, Message: 'a + Clone>(tape: &Tape) -> Column<'a, Message> {
    let margin_left =
        (tape.get_first_char_position() as isize + tape.get_cursor_position()) as usize;
    ui_column![ui_column![
        text(tape.get_chars_with_margin())
            .size(TAPE_TEXT_SIZE)
            .font(TAPE_FONT),
        text(format!("{}^", &SPACES[0..margin_left]))
            .size(TAPE_TEXT_SIZE)
            .font(TAPE_FONT),
    ]]
    .align_items(iced::Alignment::Center)
    .width(iced::Length::Fill)
}
