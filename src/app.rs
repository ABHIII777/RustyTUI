use crate::core::system::{SystemMonitor, SystemStats};
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
    pub system: SystemMonitor,
    pub system_stats: SystemStats,
}

impl App {
    pub fn new() -> Self {
        let mut system = SystemMonitor::new();
        let stats = system.update();

        Self {
            running: true,
            mode: Mode::Dashboard,
            tick: 0,
            system,
            system_stats: stats,
        }
    }

    pub fn update(&mut self) {
        self.system_stats = self.system.update();
        self.tick += 1;
    }
}
