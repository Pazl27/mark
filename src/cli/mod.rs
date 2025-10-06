pub mod parser;

use crate::config::{get_default_config_path, ConfigLoader, MarkConfig};
use crate::error::Result;

use crate::ui::{self, App};
use clap::Parser;

/// Initialize and run the CLI application
pub fn run() -> Result<()> {
    let cli = parser::Cli::parse();

    let config_path = if let Some(path) = &cli.config {
        path.clone()
    } else {
        get_default_config_path()?
    };

    let loader = ConfigLoader::with_path(config_path)?;
    let config = loader.config();

    if cli.all {
        let directory = cli.file
            .as_ref()
            .and_then(|p| p.to_str())
            .unwrap_or(".");
        launch_file_browser(directory, &config, true)?;
        return Ok(());
    }

    // Handle the file argument
    match cli.file {
        Some(path) => {
            if path.is_file() {
                println!("Opening file: {}", path.display());
                // TODO: Launch markdown viewer with the specific file
                Ok(())
            } else if path.is_dir() {
        launch_file_browser(path.to_str().unwrap(), &config, false)?;
                Ok(())
            } else {
                eprintln!("Error: Path does not exist: {}", path.display());
                Ok(())
            }
        }
        None => {
            // Browse current directory
            launch_file_browser(".", &config, false)?;
            Ok(())
        }
    }
}

fn launch_file_browser(directory: &str, config: &MarkConfig, show_all: bool) -> Result<()> {
    // Initialize terminal
    let mut terminal = ui::init()?;
    
    // Create and run the app
    let result = run_app(directory, &mut terminal, config, show_all);
    
    // Always restore terminal, even if there was an error
    ui::restore()?;
    
    match result {
        Ok(Some(file)) => {
            println!("Selected file: {}", file.path.display());
            // TODO: Launch markdown viewer with the selected file
            Ok(())
        }
        Ok(None) => {
            // User quit without selecting a file
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn run_app(directory: &str, terminal: &mut crate::ui::Tui, config: &MarkConfig, show_all: bool) -> Result<Option<crate::search::MarkdownFile>> {
    let mut app = App::new(directory, config, show_all)?;
    app.run(terminal)
}
