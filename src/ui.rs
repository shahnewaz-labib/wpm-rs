use crate::app::{App, GameState, ViewState, TEST_MODES, TypedChar};
use crate::ascii_font;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Sparkline, Table},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App) {
    match app.view_state {
        ViewState::Typing => draw_typing_screen(frame, app),
        ViewState::Settings => draw_settings_screen(frame, app),
        ViewState::History => draw_history_screen(frame, app),
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

    draw_title(frame, chunks[0], app);
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
        .style(Style::default().fg(app.theme.title()).bold())
        .centered()
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // Settings content
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Settings ");
    let inner = block.inner(chunks[1]);
    frame.render_widget(block, chunks[1]);

    // Calculate vertical centering for options
    let options_height = TEST_MODES.len() as u16 + 10; // label + modes + spacer + 7 toggles
    let vertical_offset = inner.height.saturating_sub(options_height) / 2;

    let content_area = Rect {
        x: inner.x,
        y: inner.y + vertical_offset,
        width: inner.width,
        height: inner.height.saturating_sub(vertical_offset),
    };

    let mut constraints = vec![Constraint::Length(2)]; // "Select test mode:" label
    for _ in TEST_MODES {
        constraints.push(Constraint::Length(1));
    }
    constraints.push(Constraint::Length(1)); // Spacer
    for _ in 0..7 {
        constraints.push(Constraint::Length(1)); // Toggle line
    }
    let option_chunks = Layout::vertical(constraints).split(content_area);

    // Label
    let label = Paragraph::new("Select test mode:")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    frame.render_widget(label, option_chunks[0]);

    // Mode options
    for (i, mode) in TEST_MODES.iter().enumerate() {
        let is_selected = i == app.settings_cursor;
        let is_current = *mode == app.test_mode;

        let (prefix, suffix) = if is_selected {
            ("> ", " <")
        } else {
            ("  ", "  ")
        };

        let current_marker = if is_current { " (current)" } else { "" };

        let style = if is_selected {
            Style::default().fg(app.theme.selected()).bold()
        } else if is_current {
            Style::default().fg(app.theme.current())
        } else {
            Style::default().fg(app.theme.unselected())
        };

        let line = Paragraph::new(format!("{}{}{}{}", prefix, mode.label(), suffix, current_marker))
            .style(style)
            .alignment(Alignment::Center);
        frame.render_widget(line, option_chunks[i + 1]);
    }

    // Toggle lines (compact format: "Label: Value")
    let toggles: [(&str, &str, bool); 7] = [
        ("Punctuation", if app.punctuation { "On" } else { "Off" }, app.is_on_punctuation_toggle()),
        ("Text Source", app.text_source.label(), app.is_on_text_source_toggle()),
        ("Language", app.language.label(), app.is_on_language_toggle()),
        ("Blind Mode", if app.blind_mode { "On" } else { "Off" }, app.is_on_blind_mode_toggle()),
        ("Stop on Error", if app.stop_on_error { "On" } else { "Off" }, app.is_on_stop_on_error_toggle()),
        ("Theme", app.theme.label(), app.is_on_theme_toggle()),
        ("Sound", if app.sound_enabled { "On" } else { "Off" }, app.is_on_sound_toggle()),
    ];

    for (idx, (label, value, is_selected)) in toggles.iter().enumerate() {
        let (prefix, suffix) = if *is_selected {
            ("> ", " <")
        } else {
            ("  ", "  ")
        };
        let style = if *is_selected {
            Style::default().fg(app.theme.selected()).bold()
        } else {
            Style::default().fg(app.theme.current())
        };
        let line = Paragraph::new(format!("{}{}: {}{}", prefix, label, value, suffix))
            .style(style)
            .alignment(Alignment::Center);
        frame.render_widget(line, option_chunks[TEST_MODES.len() + 1 + idx]);
    }

    // Help
    let help = Paragraph::new(" [Up/Down] Navigate | [Enter] Select/Toggle | [Tab/Esc] Back")
        .style(Style::default().fg(app.theme.help()))
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(help, chunks[2]);
}

fn draw_history_screen(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let chunks = Layout::vertical([
        Constraint::Length(3), // Title
        Constraint::Min(5),    // Table
        Constraint::Length(4), // Sparkline
        Constraint::Length(3), // Help
    ])
    .split(area);

    // Title
    let title = Paragraph::new("History")
        .style(Style::default().fg(app.theme.title()).bold())
        .centered()
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // Table of past runs
    let entries = app.history.entries();
    let rows = entries.iter().rev().take(20).enumerate().map(|(i, entry)| {
        let num = entries.len() - i;
        Row::new(vec![
            Cell::from(format!("{}", num)),
            Cell::from(entry.test_mode.label()),
            Cell::from(format!("{:.0}", entry.wpm)),
            Cell::from(format!("{:.0}", entry.raw_wpm)),
            Cell::from(format!("{:.1}%", entry.accuracy)),
        ])
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(5),
            Constraint::Min(15),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(10),
        ],
    )
    .header(
        Row::new(vec!["#", "Mode", "Net WPM", "Raw WPM", "Accuracy"])
            .style(Style::default().fg(app.theme.title()).bold()),
    )
    .block(Block::default().borders(Borders::ALL).title(" Recent Runs "));

    frame.render_widget(table, chunks[1]);

    // WPM trend sparkline
    let wpm_data: Vec<u64> = entries.iter().rev().map(|e| e.wpm.round() as u64).collect();
    let sparkline = Sparkline::default()
        .data(&wpm_data)
        .style(Style::default().fg(app.theme.wpm()))
        .block(Block::default().borders(Borders::ALL).title(" WPM Trend "));
    frame.render_widget(sparkline, chunks[2]);

    // Help
    let help = Paragraph::new(" [F2/Esc/Tab/Enter] Back to typing")
        .style(Style::default().fg(app.theme.help()))
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(help, chunks[3]);
}

fn draw_title(frame: &mut Frame, area: Rect, app: &App) {
    let title = Paragraph::new("WPM Typing Test")
        .style(Style::default().fg(app.theme.title()).bold())
        .centered()
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(title, area);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum WordStatus {
    Correct,
    HasErrors,
    InProgress,
    Untyped,
}

fn build_word_status_map(typed_chars: &[TypedChar]) -> Vec<WordStatus> {
    let n = typed_chars.len();
    let mut status_map = vec![WordStatus::Untyped; n];

    let mut word_start: Option<usize> = None;
    for i in 0..n {
        if typed_chars[i].expected == " " {
            if let Some(start) = word_start {
                let status = compute_word_status(typed_chars, start, i);
                status_map[start..i].fill(status);
                status_map[i] = status;
                word_start = None;
            }
        } else if word_start.is_none() {
            word_start = Some(i);
        }
    }
    if let Some(start) = word_start {
        let status = compute_word_status(typed_chars, start, n);
        status_map[start..n].fill(status);
    }

    status_map
}

fn compute_word_status(typed_chars: &[TypedChar], start: usize, end: usize) -> WordStatus {
    let all_typed = (start..end).all(|i| typed_chars[i].is_typed());
    if !all_typed {
        let any_typed = (start..end).any(|i| typed_chars[i].is_typed());
        return if any_typed {
            WordStatus::InProgress
        } else {
            WordStatus::Untyped
        };
    }
    let all_correct = (start..end).all(|i| typed_chars[i].is_correct());
    if all_correct {
        WordStatus::Correct
    } else {
        WordStatus::HasErrors
    }
}

fn draw_text_area(frame: &mut Frame, area: Rect, app: &App) {
    if app.state == GameState::Finished {
        draw_finished_screen(frame, area, app);
        return;
    }

    let status_map = build_word_status_map(&app.typed_chars);
    let mut spans: Vec<Span> = Vec::new();

    for (i, typed_char) in app.typed_chars.iter().enumerate() {
        let is_cursor = i == app.cursor_pos;

        let style = if app.blind_mode {
            if is_cursor {
                Style::default()
                    .fg(app.theme.cursor())
                    .add_modifier(Modifier::UNDERLINED | Modifier::BOLD)
            } else {
                Style::default().fg(app.theme.untyped())
            }
        } else {
            let word_status = status_map[i];
            match word_status {
                WordStatus::Correct => Style::default().fg(app.theme.correct()),
                WordStatus::HasErrors => Style::default().fg(app.theme.wrong()),
                WordStatus::InProgress | WordStatus::Untyped => {
                    if typed_char.is_typed() {
                        if typed_char.is_correct() {
                            Style::default().fg(app.theme.correct())
                        } else {
                            Style::default()
                                .fg(app.theme.wrong())
                                .add_modifier(Modifier::CROSSED_OUT)
                        }
                    } else if is_cursor {
                        Style::default()
                            .fg(app.theme.cursor())
                            .add_modifier(Modifier::UNDERLINED | Modifier::BOLD)
                    } else {
                        Style::default().fg(app.theme.untyped())
                    }
                }
            }
        };

        spans.push(Span::styled(typed_char.expected.as_str(), style));
    }

    let text = Paragraph::new(Line::from(spans))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(
                    " Type the text below ({}, {}) ",
                    app.test_mode.label(),
                    app.source_label()
                )),
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
    let net = app.net_wpm();
    let raw = app.wpm();
    let wpm_int = net.round() as u32;
    let color = score_color(net);
    let message = score_message(net);

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
        let extra = if app.export_message.is_some() { 2 } else { 0 };
        let content_height = ascii_font::FONT_HEIGHT + 8 + extra; // ASCII + WPM label + message + spacer + stats + spacer + 2 heat-map lines + optional export msg
        let vertical_offset = inner.height.saturating_sub(content_height as u16) / 2;

        let content_area = Rect {
            x: inner.x,
            y: inner.y + vertical_offset,
            width: inner.width,
            height: inner.height.saturating_sub(vertical_offset),
        };

        let mut constraints = vec![
            Constraint::Length(ascii_font::FONT_HEIGHT as u16), // ASCII art
            Constraint::Length(1), // WPM label
            Constraint::Length(1), // Message
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Stats
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Slowest keys
            Constraint::Length(1), // Error-prone keys
        ];
        if app.export_message.is_some() {
            constraints.push(Constraint::Length(1)); // Spacer
            constraints.push(Constraint::Length(1)); // Export message
        }
        let chunks = Layout::vertical(constraints).split(content_area);

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
        let mut stats_spans = vec![
            Span::raw("Net: "),
            Span::styled(
                format!("{:.0} WPM", net),
                Style::default().fg(Color::Green),
            ),
            Span::raw("  |  Raw: "),
            Span::styled(
                format!("{:.0} WPM", raw),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw("  |  Accuracy: "),
            Span::styled(
                format!("{:.1}%", app.accuracy()),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw("  |  Time: "),
            Span::styled(
                format!("{:.1}s", app.elapsed_secs()),
                Style::default().fg(Color::Cyan),
            ),
        ];

        if let Some(best) = app.history.best_wpm(app.test_mode) {
            stats_spans.push(Span::raw("  |  Best: "));
            stats_spans.push(Span::styled(
                format!("{:.0} WPM", best),
                Style::default().fg(app.theme.best()),
            ));
        }

        let stats_line = Line::from(stats_spans);
        let stats = Paragraph::new(stats_line).alignment(Alignment::Center);
        frame.render_widget(stats, chunks[4]);

        // Key heat-map
        let slowest = app.slowest_keys(5);
        let slowest_line = if slowest.is_empty() {
            Line::from("")
        } else {
            let mut spans = vec![Span::raw("Slowest: ")];
            for (i, (key, stat)) in slowest.iter().enumerate() {
                if i > 0 {
                    spans.push(Span::raw(" | "));
                }
                let display = if *key == ' ' { "spc".to_string() } else { key.to_string() };
                spans.push(Span::styled(
                    format!("{} ({:.0}ms)", display, stat.avg_time_ms()),
                    Style::default().fg(Color::Yellow),
                ));
            }
            Line::from(spans)
        };
        let slowest_para = Paragraph::new(slowest_line).alignment(Alignment::Center);
        frame.render_widget(slowest_para, chunks[6]);

        let errors = app.most_error_prone_keys(5);
        let error_line = if errors.is_empty() {
            Line::from("")
        } else {
            let mut spans = vec![Span::raw("Errors: ")];
            for (i, (key, stat)) in errors.iter().enumerate() {
                if i > 0 {
                    spans.push(Span::raw(" | "));
                }
                let display = if *key == ' ' { "spc".to_string() } else { key.to_string() };
                spans.push(Span::styled(
                    format!("{} ({})", display, stat.error_count),
                    Style::default().fg(Color::Red),
                ));
            }
            Line::from(spans)
        };
        let error_para = Paragraph::new(error_line).alignment(Alignment::Center);
        frame.render_widget(error_para, chunks[7]);

        // Export message
        if let Some(ref msg) = app.export_message {
            let export_para = Paragraph::new(Line::from(Span::styled(
                msg.as_str(),
                Style::default().fg(Color::Magenta),
            )))
            .alignment(Alignment::Center);
            frame.render_widget(export_para, chunks[9]);
        }
    } else {
        // Fallback for narrow terminals
        let mut stats_spans = vec![
            Span::raw("Net: "),
            Span::styled(
                format!("{:.0} WPM", net),
                Style::default().fg(Color::Green),
            ),
            Span::raw(" | Raw: "),
            Span::styled(
                format!("{:.0} WPM", raw),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(" | Accuracy: "),
            Span::styled(
                format!("{:.1}%", app.accuracy()),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(" | Time: "),
            Span::styled(
                format!("{:.1}s", app.elapsed_secs()),
                Style::default().fg(Color::Cyan),
            ),
        ];

        if let Some(best) = app.history.best_wpm(app.test_mode) {
            stats_spans.push(Span::raw(" | Best: "));
            stats_spans.push(Span::styled(
                format!("{:.0} WPM", best),
                Style::default().fg(app.theme.best()),
            ));
        }

        let mut lines = vec![
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
            Line::from(stats_spans),
        ];

        // Key heat-map for narrow terminal
        let slowest = app.slowest_keys(3);
        if !slowest.is_empty() {
            let parts: Vec<String> = slowest
                .iter()
                .map(|(key, stat)| {
                    let display = if *key == ' ' { "spc".to_string() } else { key.to_string() };
                    format!("{} ({:.0}ms)", display, stat.avg_time_ms())
                })
                .collect();
            lines.push(Line::from(format!("Slowest: {}", parts.join(" | "))));
        }

        let errors = app.most_error_prone_keys(3);
        if !errors.is_empty() {
            let parts: Vec<String> = errors
                .iter()
                .map(|(key, stat)| {
                    let display = if *key == ' ' { "spc".to_string() } else { key.to_string() };
                    format!("{} ({})", display, stat.error_count)
                })
                .collect();
            lines.push(Line::from(format!("Errors: {}", parts.join(" | "))));
        }

        let content = Paragraph::new(lines).alignment(Alignment::Center);
        frame.render_widget(content, inner);
    }
}

fn draw_stats(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(2), // WPM sparkline
        Constraint::Length(2), // Stats line
        Constraint::Length(1), // Help line
    ])
    .split(area);

    // WPM sparkline
    let sparkline = Sparkline::default()
        .block(Block::default().borders(Borders::TOP).title(" WPM Trend "))
        .data(&app.wpm_samples)
        .style(Style::default().fg(app.theme.wpm()));
    frame.render_widget(sparkline, chunks[0]);

    // Stats line
    let (time_label, time) = if app.test_mode.is_timed() {
        (" Time: ", format!("{:.0}s", app.time_remaining()))
    } else {
        (" Time: ", format!("{:.1}s", app.elapsed_secs()))
    };

    let wpm = format!("{:.0}", app.net_wpm());
    let accuracy = format!("{:.1}%", app.accuracy());

    let stats_line = Line::from(vec![
        Span::raw(time_label),
        Span::styled(&time, Style::default().fg(app.theme.time())),
        Span::raw(" | Net WPM: "),
        Span::styled(&wpm, Style::default().fg(app.theme.wpm()).bold()),
        Span::raw(" | Accuracy: "),
        Span::styled(&accuracy, Style::default().fg(app.theme.accuracy())),
    ]);

    let stats = Paragraph::new(stats_line);
    frame.render_widget(stats, chunks[1]);

    // Help line based on state
    let help_text = match app.state {
        GameState::NotStarted => " [Tab] Settings | [F2] History | Start typing to begin...",
        GameState::Running => " [Backspace] Delete | [Esc] Quit",
        GameState::Finished => " [Space] Play Again | [E] Export | [Tab] Settings | [F2] History | [Esc] Quit",
    };

    let help = Paragraph::new(help_text).style(Style::default().fg(app.theme.help()));
    frame.render_widget(help, chunks[2]);
}
