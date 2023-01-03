use crate::{constants::DEFAULT_STATE, table::Table, tape::Tape, task::Direction};

pub struct Machine {
    // Surrent state of the machine
    state: usize,

    // Tape which gets modified by the machine
    tape: Tape,

    // Current step
    step: usize,

    // True if machine has finished its work
    is_halted: bool,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            state: DEFAULT_STATE,
            tape: Tape::new(),
            step: 0,
            is_halted: false,
        }
    }

    pub fn reset(&mut self, new_tape: Tape) {
        self.state = DEFAULT_STATE;
        self.tape = new_tape;
        self.step = 0;
        self.is_halted = false
    }

    pub fn is_halted(&self) -> bool {
        self.is_halted
    }

    pub fn get_step(&self) -> usize {
        self.step
    }

    pub fn get_state(&self) -> usize {
        self.state
    }

    pub fn get_tape(&self) -> &Tape {
        &self.tape
    }

    pub fn next_step(&mut self, table: &Table) {
        if self.is_halted {
            return;
        }

        let current_char = self.tape.get_current_char();
        let task = table.get_task(self.state, current_char);

        if let Some(task) = task {
            self.tape.set_current_char(task.character);
            self.state = task.state;

            let move_result = match task.direction {
                Direction::Left => self.tape.move_cursor_left(),
                Direction::Right => self.tape.move_cursor_right(),
                Direction::Stop => Err(()),
            };

            if let Err(_) = move_result {
                self.is_halted = true;
            }
        } else {
            self.is_halted = true;
        };

        self.step += 1;
    }
}
