mod cli;
mod error;

use error::{CliError, Result};
use std::error::Error;
use std::process;

#[tokio::main]
async fn main() {
    if let Err(e) = run_application().await {
        handle_error(e);
    }
}

async fn run_application() -> Result<()> {
    cli::run().await?;
    Ok(())
}

fn handle_error(error: CliError) {
    eprintln!("Error: {}", error);

    let mut source = error.source();
    while let Some(err) = source {
        eprintln!("  Caused by: {}", err);
        source = err.source();
    }

    print_error_suggestions(&error);

    process::exit(error.exit_code());
}

fn print_error_suggestions(error: &CliError) {
    match error {
        CliError::FileNotFound { path } => {
            eprintln!();
            eprintln!("Suggestions:");
            eprintln!("  • Check if the file path is correct: {}", path.display());
            eprintln!("  • Ensure the file exists and you have read permissions");
        }
        CliError::InvalidFileFormat { path } => {
            eprintln!();
            eprintln!("Suggestions:");
            eprintln!("  • Make sure '{}' is a markdown file", path.display());
            eprintln!("  • Supported extensions: .md, .markdown");
        }
        CliError::InvalidWidth { .. } => {
            eprintln!();
            eprintln!("Suggestions:");
            eprintln!("  • Use a width value between 20 and 200 characters");
            eprintln!("  • Example: mark --width 80 README.md");
        }
        CliError::Config { .. } => {
            eprintln!();
            eprintln!("Suggestions:");
            eprintln!("  • Check your configuration file syntax");
            eprintln!("  • Ensure the config file path is correct");
        }
        _ => {
            eprintln!();
            eprintln!("For more help, run: mark --help");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_error_exit_codes() {
        let file_not_found = CliError::FileNotFound {
            path: PathBuf::from("nonexistent.md"),
        };
        assert_eq!(file_not_found.exit_code(), 2);

        let invalid_width = CliError::InvalidWidth { width: 300 };
        assert_eq!(invalid_width.exit_code(), 22);
    }
}
