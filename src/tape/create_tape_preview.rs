use crate::constants::{MAX_TAPE_FONT_SIZE, MAX_TAPE_LENGTH, MIN_TAPE_FONT_SIZE, TAPE_FONT};
use iced::widget::{column as ui_column, text, Column};
use std::iter::repeat;

use super::Tape;

pub fn create_tape_preview<'a, Message: 'a + Clone>(tape: &Tape) -> Column<'a, Message> {
    let min_max_font_size_difference = MAX_TAPE_FONT_SIZE - MIN_TAPE_FONT_SIZE;
    let font_size = MAX_TAPE_FONT_SIZE
        - (min_max_font_size_difference * tape.get_length() as u16) / (MAX_TAPE_LENGTH as u16 - 1);

    let cursor_position =
        (tape.get_first_char_position() as isize + tape.get_cursor_position()) as usize;
    let margin_left: String = repeat(' ').take(cursor_position).collect();

    ui_column![ui_column![
        text(tape.get_chars_with_margin())
            .size(font_size)
            .font(TAPE_FONT),
        text(format!("{}^", margin_left))
            .size(font_size)
            .font(TAPE_FONT),
    ]]
    .align_items(iced::Alignment::Center)
    .width(iced::Length::Fill)
    .height(iced::Length::Units(50))
}
