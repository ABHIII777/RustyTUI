#[derive(Debug, Clone, Copy)]

pub enum Mode {
    Dashboard,
    Music,
    Focus,
    Git,
    Chat,
    Art
}

pub struct App {
    pub running: bool,
    pub mode: Mode,
    pub tick: u64,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            mode: Mode::Dashboard,
            tick: 0,
        }
    }

    pub fn next_tick(&mut self) {
        self.tick += 1;
    }
}
