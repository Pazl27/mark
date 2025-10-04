use mark::{
    cli,
    error::{MarkError, Result},
};
use std::error::Error;
use std::process;

fn main() {
    if let Err(e) = run_application() {
        handle_error(e);
    }
}

fn run_application() -> Result<()> {
    cli::run()?;
    Ok(())
}

fn handle_error(error: MarkError) {
    eprintln!("Error: {}", error);

    let mut source = error.source();
    while let Some(err) = source {
        eprintln!("  Caused by: {}", err);
        source = err.source();
    }

    print_error_suggestions(&error);

    process::exit(error.exit_code());
}

fn print_error_suggestions(error: &MarkError) {
    match error {
        MarkError::FileNotFound { path } => {
            eprintln!();
            eprintln!("Suggestions:");
            eprintln!("  • Check if the file path is correct: {}", path.display());
            eprintln!("  • Ensure the file exists and you have read permissions");
        }
        MarkError::InvalidFileFormat { path } => {
            eprintln!();
            eprintln!("Suggestions:");
            eprintln!("  • Make sure '{}' is a markdown file", path.display());
            eprintln!("  • Supported extensions: .md, .markdown");
        }
        MarkError::InvalidWidth { .. } => {
            eprintln!();
            eprintln!("Suggestions:");
            eprintln!("  • Use a width value between 20 and 200 characters");
            eprintln!("  • Example: mark --width 80 README.md");
        }
        MarkError::Config { .. } => {
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
        let file_not_found = MarkError::FileNotFound {
            path: PathBuf::from("nonexistent.md"),
        };
        assert_eq!(file_not_found.exit_code(), 2);

        let invalid_width = MarkError::InvalidWidth { width: 300 };
        assert_eq!(invalid_width.exit_code(), 22);
    }
}
