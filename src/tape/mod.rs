pub mod create_tape_preview;

use crate::constants::{EMPTY_CHAR, TAPE_CHARS_NUMBER};
use std::iter::repeat;

#[derive(Clone)]
pub struct Tape {
    /// String with len == TAPE_CHARS_NUMBER, with characters in the middle
    /// set using `set_chars` and filled with EMPTY_CHARs on left and right end
    chars: String,

    /// Position of cursor relative to `first_char_position`
    cursor_position: isize,

    /// Index of the first of characters in `chars` set using `set_chars`
    first_char_position: isize,

    /// Index of the last of characters in `chars` set using `set_chars`
    last_char_position: isize,
}

impl Tape {
    pub fn new() -> Self {
        let position_zero = TAPE_CHARS_NUMBER as isize / 2;

        Self {
            chars: repeat(EMPTY_CHAR).take(TAPE_CHARS_NUMBER).collect(),
            cursor_position: 0,
            first_char_position: position_zero,
            last_char_position: position_zero - 1,
        }
    }

    pub fn get_chars_with_margin(&self) -> &String {
        &self.chars
    }

    pub fn get_chars_without_margin(&self) -> &str {
        let range = self.first_char_position as usize..=self.last_char_position as usize;
        &self.chars[range]
    }

    pub fn set_chars(&mut self, mut new_chars: String) {
        if new_chars.len() > TAPE_CHARS_NUMBER {
            new_chars.drain(TAPE_CHARS_NUMBER..);
        }

        let first_char_position = (TAPE_CHARS_NUMBER - new_chars.len()) / 2;
        let last_char_position = first_char_position + new_chars.len() - 1;
        let replace_range = first_char_position..=last_char_position;

        self.chars = repeat(EMPTY_CHAR).take(TAPE_CHARS_NUMBER).collect();
        self.chars.replace_range(replace_range, &new_chars);

        self.first_char_position = first_char_position as isize;
        self.last_char_position = last_char_position as isize;
        self.set_cursor_position(self.cursor_position);
    }

    pub fn get_cursor_position(&self) -> isize {
        self.cursor_position
    }

    pub fn set_cursor_position(&mut self, new_cursor_position: isize) {
        let min_position = -self.first_char_position;
        let max_position = min_position + TAPE_CHARS_NUMBER as isize - 1;

        self.cursor_position = if new_cursor_position <= min_position {
            min_position
        } else if new_cursor_position >= max_position {
            max_position
        } else {
            new_cursor_position
        };
    }

    pub fn get_first_char_position(&self) -> isize {
        self.first_char_position
    }

    pub fn get_current_char(&self) -> char {
        self.chars
            .chars()
            .nth((self.first_char_position + self.cursor_position) as usize)
            .unwrap()
    }

    pub fn set_current_char(&mut self, new_char: char) {
        let position = (self.first_char_position + self.cursor_position) as usize;
        self.chars
            .replace_range(position..=position, &format!("{}", new_char))
    }

    pub fn move_cursor_left(&mut self) -> Result<(), ()> {
        if self.first_char_position + self.cursor_position - 1 >= 0 {
            self.cursor_position -= 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn move_cursor_right(&mut self) -> Result<(), ()> {
        if self.first_char_position + self.cursor_position + 1 < TAPE_CHARS_NUMBER as isize {
            self.cursor_position += 1;
            Ok(())
        } else {
            Err(())
        }
    }
}
