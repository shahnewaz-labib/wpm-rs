use crate::app::{App, GameState, ViewState, WORD_COUNT_OPTIONS};
use crate::ascii_font;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App) {
    match app.view_state {
        ViewState::Typing => draw_typing_screen(frame, app),
        ViewState::Settings => draw_settings_screen(frame, app),
    }
}

fn draw_typing_screen(frame: &mut Frame, app: &App) {
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

fn draw_settings_screen(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let chunks = Layout::vertical([
        Constraint::Length(3), // Title
        Constraint::Min(5),    // Settings content
        Constraint::Length(3), // Help
    ])
    .split(area);

    // Title
    let title = Paragraph::new("Settings")
        .style(Style::default().fg(Color::Cyan).bold())
        .centered()
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // Settings content
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Word Count ");
    let inner = block.inner(chunks[1]);
    frame.render_widget(block, chunks[1]);

    // Calculate vertical centering for options
    let options_height = WORD_COUNT_OPTIONS.len() as u16 + 2; // options + label + spacer
    let vertical_offset = inner.height.saturating_sub(options_height) / 2;

    let content_area = Rect {
        x: inner.x,
        y: inner.y + vertical_offset,
        width: inner.width,
        height: inner.height.saturating_sub(vertical_offset),
    };

    let mut constraints = vec![Constraint::Length(2)]; // "Select word count:" label
    for _ in WORD_COUNT_OPTIONS {
        constraints.push(Constraint::Length(1));
    }
    let option_chunks = Layout::vertical(constraints).split(content_area);

    // Label
    let label = Paragraph::new("Select word count:")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    frame.render_widget(label, option_chunks[0]);

    // Options
    for (i, &count) in WORD_COUNT_OPTIONS.iter().enumerate() {
        let is_selected = i == app.settings_cursor;
        let is_current = count == app.word_count;

        let (prefix, suffix) = if is_selected {
            ("> ", " <")
        } else {
            ("  ", "  ")
        };

        let current_marker = if is_current { " (current)" } else { "" };

        let style = if is_selected {
            Style::default().fg(Color::Yellow).bold()
        } else if is_current {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let line = Paragraph::new(format!("{}{}{}{}", prefix, count, suffix, current_marker))
            .style(style)
            .alignment(Alignment::Center);
        frame.render_widget(line, option_chunks[i + 1]);
    }

    // Help
    let help = Paragraph::new(" [Up/Down] Navigate | [Enter] Select | [Tab/Esc] Back")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(help, chunks[2]);
}

fn draw_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("WPM Typing Test")
        .style(Style::default().fg(Color::Cyan).bold())
        .centered()
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(title, area);
}

fn draw_text_area(frame: &mut Frame, area: Rect, app: &App) {
    if app.state == GameState::Finished {
        draw_finished_screen(frame, area, app);
        return;
    }

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

    let text = Paragraph::new(Line::from(spans))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Type the text below ({} words) ", app.word_count)),
        )
        .wrap(ratatui::widgets::Wrap { trim: false });

    frame.render_widget(text, area);
}

fn score_color(wpm: f64) -> Color {
    if wpm >= 80.0 {
        Color::Cyan
    } else if wpm >= 60.0 {
        Color::Green
    } else if wpm >= 40.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}

fn score_message(wpm: f64) -> &'static str {
    if wpm >= 80.0 {
        "Excellent!"
    } else if wpm >= 60.0 {
        "Great job!"
    } else if wpm >= 40.0 {
        "Good effort!"
    } else {
        "Keep practicing!"
    }
}

fn draw_finished_screen(frame: &mut Frame, area: Rect, app: &App) {
    let wpm = app.wpm();
    let wpm_int = wpm.round() as u32;
    let color = score_color(wpm);
    let message = score_message(wpm);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Results ");

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Check if we have enough width for ASCII art
    let ascii_width = ascii_font::number_width(wpm_int);
    let use_ascii = inner.width as usize >= ascii_width + 4;

    if use_ascii {
        let ascii_lines = ascii_font::render_number(wpm_int);

        // Calculate vertical centering
        let content_height = ascii_font::FONT_HEIGHT + 4; // ASCII + WPM label + message + stats
        let vertical_offset = inner.height.saturating_sub(content_height as u16) / 2;

        let content_area = Rect {
            x: inner.x,
            y: inner.y + vertical_offset,
            width: inner.width,
            height: inner.height.saturating_sub(vertical_offset),
        };

        let chunks = Layout::vertical([
            Constraint::Length(ascii_font::FONT_HEIGHT as u16), // ASCII art
            Constraint::Length(1), // WPM label
            Constraint::Length(1), // Message
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Stats
        ])
        .split(content_area);

        // Render ASCII art lines
        let ascii_paragraph = Paragraph::new(
            ascii_lines
                .into_iter()
                .map(|line| Line::from(Span::styled(line, Style::default().fg(color).bold())))
                .collect::<Vec<_>>(),
        )
        .alignment(Alignment::Center);
        frame.render_widget(ascii_paragraph, chunks[0]);

        // WPM label
        let wpm_label = Paragraph::new(Line::from(Span::styled(
            "WPM",
            Style::default().fg(color).bold(),
        )))
        .alignment(Alignment::Center);
        frame.render_widget(wpm_label, chunks[1]);

        // Message
        let msg = Paragraph::new(Line::from(Span::styled(
            message,
            Style::default().fg(Color::White).add_modifier(Modifier::ITALIC),
        )))
        .alignment(Alignment::Center);
        frame.render_widget(msg, chunks[2]);

        // Stats line
        let stats_line = Line::from(vec![
            Span::raw("Accuracy: "),
            Span::styled(
                format!("{:.1}%", app.accuracy()),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw("  |  Time: "),
            Span::styled(
                format!("{:.1}s", app.elapsed_secs()),
                Style::default().fg(Color::Cyan),
            ),
        ]);
        let stats = Paragraph::new(stats_line).alignment(Alignment::Center);
        frame.render_widget(stats, chunks[4]);
    } else {
        // Fallback for narrow terminals
        let lines = vec![
            Line::from(Span::styled(
                format!("{} WPM", wpm_int),
                Style::default().fg(color).bold().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                message,
                Style::default().fg(Color::White).add_modifier(Modifier::ITALIC),
            )),
            Line::from(""),
            Line::from(vec![
                Span::raw("Accuracy: "),
                Span::styled(
                    format!("{:.1}%", app.accuracy()),
                    Style::default().fg(Color::Yellow),
                ),
                Span::raw(" | Time: "),
                Span::styled(
                    format!("{:.1}s", app.elapsed_secs()),
                    Style::default().fg(Color::Cyan),
                ),
            ]),
        ];

        let content = Paragraph::new(lines).alignment(Alignment::Center);
        frame.render_widget(content, inner);
    }
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
        Span::raw(" | WPM: "),
        Span::styled(&wpm, Style::default().fg(Color::Green).bold()),
        Span::raw(" | Accuracy: "),
        Span::styled(&accuracy, Style::default().fg(Color::Yellow)),
    ]);

    let stats = Paragraph::new(stats_line).block(Block::default().borders(Borders::TOP));
    frame.render_widget(stats, chunks[0]);

    // Help line based on state
    let help_text = match app.state {
        GameState::NotStarted => " [Tab] Settings | Start typing to begin...",
        GameState::Running => " [Backspace] Delete | [Esc] Quit",
        GameState::Finished => " [Space] Play Again | [Tab] Settings | [Esc] Quit",
    };

    let help = Paragraph::new(help_text).style(Style::default().fg(Color::DarkGray));
    frame.render_widget(help, chunks[1]);
}
