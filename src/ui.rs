use ratatui:: {
    layout::{Layout, Constraint, Direction},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
    widgets::{Gauge},
};

use crate::app::{App, Mode};

pub fn draw_ui(f: &mut Frame, app: &App) {
    let size = f.area();

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

            let grid = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                    Constraint::Percentage(30),
                ])
                .split(chunks[1]);

            let top_row = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(grid[0]);

            let cpu_gauge = Gauge::default()
                .block(
                    Block::default()
                        .title(" CPU ")
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Cyan)),
                )
                .gauge_style(Style::default().fg(Color::Cyan))
                .percent(stats.cpu as u16);

            f.render_widget(cpu_gauge, top_row[0]);

            let mem_percent =
                (stats.memory_used as f64 / stats.memory_total as f64 * 100.0) as u16;

            let mem_gauge = Gauge::default()
                .block(
                    Block::default()
                        .title(" MEMORY ")
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Magenta)),
                )
                .gauge_style(Style::default().fg(Color::Magenta))
                .percent(mem_percent);

            f.render_widget(mem_gauge, top_row[1]);

            let core_text = stats.per_core
                .iter()
                .enumerate()
                .map(|(i, val)| format!("Core {}: {:.1}%", i, val))
                .collect::<Vec<_>>()
                .join("\n");

            let core_panel = Paragraph::new(core_text)
                .block(
                    Block::default()
                        .title(" CORES ")
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Green)),
                );

            f.render_widget(core_panel, grid[1]);

            let clock = chrono::Local::now().format("%H:%M:%S").to_string();

            let system_text = format!(
                "Uptime: {} sec\nClock: {}\nRX: {} KB\nTX: {} KB",
                stats.uptime,
                clock,
                stats.network_rx / 1024,
                stats.network_tx / 1024
            );

            let system_panel = Paragraph::new(system_text)
                .block(
                    Block::default()
                        .title(" SYSTEM ")
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Yellow)),
                );

            f.render_widget(system_panel, grid[2]);
        }

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

    let footer = Paragraph::new(
        " [1] Dash [2] Music [3] Focus [4] git [5] Chat [6] Art | q:Quit ",
    )
    .style(Style::default().fg(Color::Green))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, chunks[2]);
}
