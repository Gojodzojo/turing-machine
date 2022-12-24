use iced::widget::{row, text, Row};

use super::Tape;

pub fn create_tape_preview<'a, Message: 'a + Clone>(tape: &Tape) -> Row<'a, Message> {
    row![text(tape.get_tape_chars())]
}
