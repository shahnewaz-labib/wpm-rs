use crate::app::{App, GameState, ViewState};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub fn handle_events(app: &mut App) -> color_eyre::Result<()> {
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            handle_key(app, key);
        }
    }
    app.check_time_expired();
    app.maybe_sample_wpm();
    Ok(())
}

fn handle_key(app: &mut App, key: KeyEvent) {
    match app.view_state {
        ViewState::Settings => handle_settings_key(app, key),
        ViewState::Typing => handle_typing_key(app, key),
        ViewState::History => handle_history_key(app, key),
    }
}

fn handle_settings_key(app: &mut App, key: KeyEvent) {
    match key.code {
        // Quit on Ctrl+C
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }

        // Navigate up
        KeyCode::Up | KeyCode::Char('k') => {
            app.settings_up();
        }

        // Navigate down
        KeyCode::Down | KeyCode::Char('j') => {
            app.settings_down();
        }

        // Apply selection
        KeyCode::Enter => {
            app.apply_settings();
        }

        // Return to typing without applying
        KeyCode::Tab | KeyCode::Esc => {
            app.view_state = ViewState::Typing;
        }

        _ => {}
    }
}

fn handle_typing_key(app: &mut App, key: KeyEvent) {
    match key.code {
        // Quit on Ctrl+C or Escape
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Esc => {
            app.should_quit = true;
        }

        // Open settings when not in active test
        KeyCode::Tab => {
            if app.state != GameState::Running {
                app.toggle_settings();
            }
        }

        // Open history when not in active test
        KeyCode::F(2) => {
            if app.state != GameState::Running {
                app.toggle_history();
            }
        }

        // Restart on Space when finished
        KeyCode::Char(' ') if app.state == GameState::Finished => {
            app.reset();
        }

        // Export result card when finished
        KeyCode::Char('e') if app.state == GameState::Finished => {
            app.export_result();
        }

        // Backspace to delete
        KeyCode::Backspace => {
            app.backspace();
        }

        // Regular character input
        KeyCode::Char(c) => {
            app.type_char(c);
        }

        _ => {}
    }
}

fn handle_history_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Esc | KeyCode::Tab | KeyCode::F(2) | KeyCode::Enter => {
            app.view_state = ViewState::Typing;
        }
        _ => {}
    }
}
