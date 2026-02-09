use ratatui:: {
    layout::{Layout, Constraint, Direction},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, Mode};

pub fn draw_ui(f: &mut Frame, app: &App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(size);

    let header = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan))
        .title("RustyTUI");

    f.render_widget(header, chunks[0]);

    let title = match app.mode {
        Mode::Dashboard => "Dashboard",
        Mode::Music => "Music",
        Mode::Focus => "Focus",
        Mode::Git => "Git",
        Mode::Chat => "Chat",
        Mode::Art=> "Art",
    };

    let glitch = if app.tick % 10 < 5 { "█▓▒░" } else { "░▒▓█" } ;

    let body = Paragraph::new(format!(
    "\n [{} Mode]\n\n System Status: Online\n Signal: Stable\n\n {}\n",
        title, glitch
    ))
    .block(
            Block::default()
                .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Magenta)),
    );

    f.render_widget(body, chunks[1]);

    let footer = Paragraph::new(
        " [1] Dash [2] Music [3] Focus [4] git [5] Chat [6] Art | q:Quit ",
    )
    .style(Style::default().fg(Color::Green))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, chunks[2]);
}
