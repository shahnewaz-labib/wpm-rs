use crate::app::{App, GameState};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Main layout: title, content, stats
    let chunks = Layout::vertical([
        Constraint::Length(3), // Title
        Constraint::Min(5),    // Text area
        Constraint::Length(5), // Stats and help
    ])
    .split(area);

    draw_title(frame, chunks[0]);
    draw_text_area(frame, chunks[1], app);
    draw_stats(frame, chunks[2], app);
}

fn draw_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("WPM Typing Test")
        .style(Style::default().fg(Color::Cyan).bold())
        .centered()
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(title, area);
}

fn draw_text_area(frame: &mut Frame, area: Rect, app: &App) {
    let mut spans: Vec<Span> = Vec::new();

    for (i, typed_char) in app.typed_chars.iter().enumerate() {
        let is_cursor = i == app.cursor_pos;

        let style = if typed_char.is_typed() {
            if typed_char.is_correct() {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Red).add_modifier(Modifier::CROSSED_OUT)
            }
        } else if is_cursor {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::UNDERLINED | Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        spans.push(Span::styled(typed_char.expected.to_string(), style));
    }

    // Show cursor at end if all typed
    if app.cursor_pos == app.typed_chars.len() && app.state == GameState::Finished {
        spans.push(Span::styled(
            " ",
            Style::default().add_modifier(Modifier::UNDERLINED),
        ));
    }

    let text = Paragraph::new(Line::from(spans))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Type the text below "),
        )
        .wrap(ratatui::widgets::Wrap { trim: false });

    frame.render_widget(text, area);
}

fn draw_stats(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(2), // Stats line
        Constraint::Length(2), // Help line
    ])
    .split(area);

    // Stats line
    let time = format!("{:.1}s", app.elapsed_secs());
    let wpm = format!("{:.0}", app.wpm());
    let accuracy = format!("{:.1}%", app.accuracy());

    let stats_line = Line::from(vec![
        Span::raw(" Time: "),
        Span::styled(&time, Style::default().fg(Color::Cyan)),
        Span::raw(" │ WPM: "),
        Span::styled(&wpm, Style::default().fg(Color::Green).bold()),
        Span::raw(" │ Accuracy: "),
        Span::styled(&accuracy, Style::default().fg(Color::Yellow)),
    ]);

    let stats = Paragraph::new(stats_line).block(Block::default().borders(Borders::TOP));
    frame.render_widget(stats, chunks[0]);

    // Help line based on state
    let help_text = match app.state {
        GameState::NotStarted => " Start typing to begin...",
        GameState::Running => " [Backspace] Delete │ [Esc] Quit",
        GameState::Finished => " [Space] Play Again │ [Esc] Quit",
    };

    let help = Paragraph::new(help_text).style(Style::default().fg(Color::DarkGray));
    frame.render_widget(help, chunks[1]);
}
