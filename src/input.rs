use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Mode};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.running = false,

        KeyCode::Char('1') => app.mode = Mode::Dashboard,
        KeyCode::Char('2') => app.mode = Mode::Music,
        KeyCode::Char('3') => app.mode = Mode::Focus,
        KeyCode::Char('4') => app.mode = Mode::Git,
        KeyCode::Char('5') => app.mode = Mode::Chat,
        KeyCode::Char('6') => app.mode = Mode::Art,

        _ => {}
    }
}
