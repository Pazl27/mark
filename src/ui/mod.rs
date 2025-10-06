pub mod app;
pub mod components;
pub mod events;
pub mod file_browser;

pub use app::App;
pub use events::{Event, EventHandler};
pub use file_browser::FileBrowser;

use crate::error::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> Result<Tui> {
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore the terminal to its original state
pub fn restore() -> Result<()> {
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    Ok(())
}
