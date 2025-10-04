pub mod parser;

use crate::config::{get_default_config_path, ConfigLoader};
use crate::error::Result;
use crate::search::find_markdown_files;
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
        search_home_directory()?;
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
                let files = find_markdown_files(path.to_str().unwrap())?;
                Ok(())
            } else {
                eprintln!("Error: Path does not exist: {}", path.display());
                Ok(())
            }
        }
        None => {
            println!("Browsing current directory for markdown files:");
            Ok(())
        }
    }
}

fn search_home_directory() -> Result<()> {
    let files = find_markdown_files("~")?;
    Ok(())
}
