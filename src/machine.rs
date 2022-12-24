use crate::constants::DEFAULT_STATE;

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
}
