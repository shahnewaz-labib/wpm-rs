use crate::app::{App, GameState};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

pub fn handle_events(app: &mut App) -> color_eyre::Result<()> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            handle_key(app, key);
        }
    }
    Ok(())
}

fn handle_key(app: &mut App, key: KeyEvent) {
    match key.code {
        // Quit on Ctrl+C or Escape
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Esc => {
            app.should_quit = true;
        }

        // Restart on Space when finished
        KeyCode::Char(' ') if app.state == GameState::Finished => {
            app.reset();
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
