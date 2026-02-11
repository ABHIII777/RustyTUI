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

    match app.mode {
        Mode::Dashboard => {

            let stats = &app.system_stats;

            let text = vec![
                Line::from(format!(
                    "\n  CPU: {:.1}%\n  RAM: {} / {} MB\n\n  STATUS: ONLINE\n",
                    stats.cpu,
                    stats.memory_used / 1024,
                    stats.memory_total / 1024
                ))
            ];

            let body = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL));

            f.render_widget(body, chunks[1]);
        },
        Mode::Music => {
            let body = Paragraph::new("Music Player - Now Playing: 'Rusty Beats' by The Code Band")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Green)),
                );
            f.render_widget(body, chunks[1]);
        },
        Mode::Focus => {
            let body = Paragraph::new("Focus Mode - Time to concentrate!")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Blue)),
                );
            f.render_widget(body, chunks[1]);
        },
        Mode::Git => {
            let body = Paragraph::new("Git Mode - Manage your repositories!")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Red)),
                );
            f.render_widget(body, chunks[1]);
        },
        Mode::Chat => {
            let body = Paragraph::new("Chat Mode - Communicate with others!")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Blue)),
                );
            f.render_widget(body, chunks[1]);
        },
        Mode::Art=> {
            let body = Paragraph::new("Art Mode - Create and explore!")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Cyan)),
                );
            f.render_widget(body, chunks[1]);
        },
    };

    // let glitch = if app.tick % 10 < 5 { "█▓▒░" } else { "░▒▓█" } ;
    // let stats = &app.system_stats;

    // let body = Paragraph::new(format!(
    // ))
    // .block(
    //         Block::default()
    //             .borders(Borders::ALL)
    //                 .style(Style::default().fg(Color::Magenta)),
    // );

    // f.render_widget(body, chunks[1]);

    let footer = Paragraph::new(
        " [1] Dash [2] Music [3] Focus [4] git [5] Chat [6] Art | q:Quit ",
    )
    .style(Style::default().fg(Color::Green))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, chunks[2]);
}
