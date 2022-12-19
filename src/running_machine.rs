use crate::constants::DEFAULT_STATE;

pub struct RunningMachine {
    pub state: usize,
    pub tape: String,
    pub cursor_position: isize,
}

impl RunningMachine {
    pub fn new(initial_tape: String, initial_cursor_position: isize) -> Self {
        Self {
            state: DEFAULT_STATE,
            tape: initial_tape,
            cursor_position: initial_cursor_position,
        }
    }
}
