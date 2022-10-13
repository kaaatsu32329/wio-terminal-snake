#[derive(Debug, Clone, Copy)]
pub enum State {
    /// Display menu (defalut)
    Menu,
    /// `false` is game over.
    Snake(bool),
}

pub struct Status {
    state: State,
}

impl Status {
    pub fn new() -> Self {
        Self { state: State::Menu }
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn get_state(&self) -> State {
        self.state
    }
}

// ToDo: Start display
// ToDo: Goal display
// ToDo: Menu display
