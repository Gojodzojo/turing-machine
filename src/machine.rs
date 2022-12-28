use crate::{constants::DEFAULT_STATE, table::Table, tape::Tape, task::Direction};

pub struct Machine {
    state: usize,
    step: usize,
    is_running: bool,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            state: DEFAULT_STATE,
            step: 0,
            is_running: false,
        }
    }

    pub fn start(&mut self) {
        self.state = DEFAULT_STATE;
        self.step = 0;
        self.is_running = true
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn get_step(&self) -> usize {
        self.step
    }

    pub fn get_state(&self) -> usize {
        self.state
    }

    pub fn next_step(&mut self, table: &Table, tape: &mut Tape) {
        let current_char = tape.get_current_char();
        let task = table.get_task(self.state, current_char);

        if let Some(task) = task {
            tape.set_current_char(task.character);
            self.state = task.state;

            let move_result = match task.direction {
                Direction::Left => tape.move_cursor_left(),
                Direction::Right => tape.move_cursor_right(),
                Direction::Stop => Err(()),
            };

            if let Err(_) = move_result {
                self.is_running = false;
            }
        } else {
            self.is_running = false;
        };

        self.step += 1;
    }
}
