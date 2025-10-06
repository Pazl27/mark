use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "mark")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A terminal-based markdown viewer")]
#[command(long_about = "Mark is a terminal-based markdown viewer built in Rust.

USAGE MODES:
  • With file:    mark README.md           - Opens the specific file directly
  • Without file: mark                     - Opens file browser for current directory
  • Browse all:   mark -a                  - Shows ALL markdown files (including hidden AND ignored) in current directory
  • Browse all:   mark -a /path/to/dir     - Shows ALL markdown files (including hidden AND ignored) in specified directory
  • Browse dir:   mark /path/to/directory  - Browse files in specified directory (respects hidden_files setting and ignored_dirs)")]
pub struct Cli {
    /// Path to markdown file or directory to browse (optional)
    pub file: Option<PathBuf>,

    /// Configuration file path
    #[arg(short = 'c', long = "config", value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Width of the text display (0 for terminal width)
    #[arg(short = 'w', long = "width", value_name = "WIDTH", default_value = "0")]
    pub width: usize,

    /// Browse ALL markdown files recursively (including hidden ones AND ignored directories - shows everything)
    #[arg(short = 'a', long = "all")]
    pub all: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        // Test basic file argument
        let cli = Cli::try_parse_from(&["mark", "test.md"]).unwrap();
        assert_eq!(cli.file, Some(PathBuf::from("test.md")));
        assert_eq!(cli.width, 0);
    }

    #[test]
    fn test_cli_with_width() {
        let cli = Cli::try_parse_from(&["mark", "--width", "120", "test.md"]).unwrap();
        assert_eq!(cli.width, 120);
        assert_eq!(cli.file, Some(PathBuf::from("test.md")));
    }

    #[test]
    fn test_cli_with_config() {
        let cli = Cli::try_parse_from(&["mark", "-c", "config.toml", "test.md"]).unwrap();
        assert_eq!(cli.config, Some(PathBuf::from("config.toml")));
    }

    #[test]
    fn test_no_file() {
        let cli = Cli::try_parse_from(&["mark"]).unwrap();
        assert_eq!(cli.file, None);
        assert_eq!(cli.width, 0);
    }
}
