pub mod create_tape_preview;

use std::iter::repeat;

use crate::constants::{EMPTY_CHAR, MAX_TAPE_CHARS_NUMBER};

pub struct Tape {
    /// Characters that the user has given as an initial tape.
    /// They do not change when the machine is turned on.
    input_chars: String,

    /// Cursor position that the user has given as an initial cursor.
    /// It does not change when the machine is turned on.
    input_cursor_position: isize,

    /// Characters that are accually displayed in the tape preview.
    /// When the machine is on, it reads and writes to this string.
    tape_chars: String,

    /// Cursor position that is accually displayed in the tape preview.
    /// When the machine is on, it is meing moved or it is used to read a character from tape_chars.
    tape_cursor_position: isize,

    /// Index in the tape_chars where the first character of input_chars is stored.
    position_zero: isize,
}

impl Tape {
    pub fn new() -> Self {
        Self {
            input_chars: "".to_string(),
            input_cursor_position: 0,
            tape_chars: repeat(EMPTY_CHAR).take(MAX_TAPE_CHARS_NUMBER).collect(),
            tape_cursor_position: 0,
            position_zero: MAX_TAPE_CHARS_NUMBER as isize / 2,
        }
    }

    pub fn get_input_chars(&self) -> &String {
        &self.input_chars
    }

    pub fn get_tape_chars(&self) -> &String {
        &self.tape_chars
    }

    pub fn set_input_chars(&mut self, new_chars: String) {
        self.input_chars = new_chars;

        if self.input_chars.len() > MAX_TAPE_CHARS_NUMBER {
            self.input_chars.drain(MAX_TAPE_CHARS_NUMBER..);
        }

        self.reset();
        self.set_input_cursor_position(self.input_cursor_position);
    }

    pub fn get_input_cursor_position(&self) -> isize {
        self.input_cursor_position
    }

    pub fn get_tape_cursor_position(&self) -> isize {
        self.tape_cursor_position
    }

    pub fn set_input_cursor_position(&mut self, input_cursor_position: isize) {
        let min_position = -self.position_zero;
        let max_position = MAX_TAPE_CHARS_NUMBER as isize - self.position_zero - 1;

        let input_cursor_position = if input_cursor_position <= min_position {
            min_position
        } else if input_cursor_position >= max_position {
            max_position
        } else {
            input_cursor_position
        };

        self.tape_cursor_position = input_cursor_position;
        self.input_cursor_position = input_cursor_position;
    }

    pub fn get_position_zero(&self) -> isize {
        self.position_zero
    }

    /// Sets tape_cursor_position to input_cursor_position,
    /// sets tape_chars to input_chars with EMPTY_CHARs added to front and back to match MAX_TAPE_CHARS_NUMBER length,
    /// sets position_zero to index in the tape_chars where the first character of input_chars is stored.
    pub fn reset(&mut self) {
        self.tape_cursor_position = self.input_cursor_position;

        let replace_start_index = (MAX_TAPE_CHARS_NUMBER - self.input_chars.len()) / 2;
        let replace_range = replace_start_index..(replace_start_index + self.input_chars.len());

        self.tape_chars = repeat(EMPTY_CHAR).take(MAX_TAPE_CHARS_NUMBER).collect();
        self.tape_chars
            .replace_range(replace_range, &self.input_chars);

        self.position_zero = replace_start_index as isize;
    }

    pub fn get_current_char(&self) -> char {
        self.tape_chars
            .chars()
            .nth((self.position_zero + self.tape_cursor_position) as usize)
            .unwrap()
    }

    pub fn set_current_char(&mut self, new_char: char) {
        let position = (self.position_zero + self.tape_cursor_position) as usize;
        self.tape_chars
            .replace_range(position..=position, &format!("{}", new_char))
    }

    pub fn move_cursor_left(&mut self) -> Result<(), ()> {
        if self.position_zero + self.tape_cursor_position - 1 >= 0 {
            self.tape_cursor_position -= 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn move_cursor_right(&mut self) -> Result<(), ()> {
        if self.position_zero + self.tape_cursor_position + 1 < MAX_TAPE_CHARS_NUMBER as isize {
            self.tape_cursor_position += 1;
            Ok(())
        } else {
            Err(())
        }
    }
}
