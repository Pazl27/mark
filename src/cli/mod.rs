pub mod parser;

use crate::error::Result;
use clap::Parser;

/// Initialize and run the CLI application
pub async fn run() -> Result<()> {
    let cli = parser::Cli::parse();

    if let Some(_config_path) = &cli.config {
        // TODO: Load configuration from file
    }

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
                println!("Browsing directory: {}", path.display());
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
    println!("Searching for markdown files in home directory...");
    Ok(())
}
