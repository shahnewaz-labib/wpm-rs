mod app;
mod ascii_font;
mod event;
mod stats;
mod ui;
mod words;

use app::App;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();

    result
}

fn run(mut terminal: ratatui::DefaultTerminal) -> Result<()> {
    let mut app = App::new()?;

    while !app.should_quit {
        terminal.draw(|frame| ui::draw(frame, &app))?;
        event::handle_events(&mut app)?;
    }

    Ok(())
}
