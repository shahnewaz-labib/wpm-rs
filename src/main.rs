mod app;
mod ascii_font;
mod cli;
mod config;
mod event;
mod history;
mod stats;
mod theme;
mod ui;
mod words;

use app::App;
use clap::Parser;
use cli::Cli;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    if cli.print_text {
        println!("{}", App::new(&cli)?.target_text());
        return Ok(());
    }

    let terminal = ratatui::init();
    let result = run(terminal, &cli);
    ratatui::restore();

    result
}

fn run(mut terminal: ratatui::DefaultTerminal, cli: &Cli) -> Result<()> {
    let mut app = App::new(cli)?;

    while !app.should_quit {
        terminal.draw(|frame| ui::draw(frame, &app))?;
        event::handle_events(&mut app)?;
    }

    Ok(())
}
