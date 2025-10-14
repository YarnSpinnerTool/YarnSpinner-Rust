//! This module is here for using ratatui to interact with the terminal and
//! crossterm to listen to input. It does not contain any code specific to yarnspinner

use std::io::Stdout;

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::backend::CrosstermBackend;

pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> anyhow::Result<Terminal> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

    let terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

    Ok(terminal)
}

pub fn restore() -> anyhow::Result<()> {
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

pub fn set_panic_hook() {
    let default_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {
        let _ = restore();

        default_hook(info)
    }));
}

pub fn poll_input() -> anyhow::Result<Option<KeyCode>> {
    if let Event::Key(key) = crossterm::event::read()?
        && key.kind == KeyEventKind::Press
    {
        return Ok(Some(key.code));
    }

    Ok(None)
}
