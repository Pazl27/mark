use std::path::PathBuf;
use thiserror::Error;

/// Application error types
#[derive(Error, Debug)]
pub enum CliError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    #[error("Invalid file format: {path}. Expected markdown file (.md, .markdown)")]
    InvalidFileFormat { path: PathBuf },

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Invalid width value: {width}. Must be between 20 and 200")]
    InvalidWidth { width: usize },
}

/// Result type alias for application results
pub type Result<T> = std::result::Result<T, CliError>;

impl CliError {
    /// Create a new configuration error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Get the exit code for this error
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::FileNotFound { .. } => 2,
            Self::InvalidFileFormat { .. } | Self::InvalidWidth { .. } => 22,
            Self::Config { .. } => 78,
            Self::Io(_) => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let path = PathBuf::from("test.txt");
        let error = CliError::FileNotFound { path: path.clone() };
        assert!(error.to_string().contains("test.txt"));
    }

    #[test]
    fn test_config_error() {
        let error = CliError::config("Invalid theme");
        assert!(error.to_string().contains("Invalid theme"));
    }

    #[test]
    fn test_exit_codes() {
        let file_not_found = CliError::FileNotFound {
            path: PathBuf::from("test.md"),
        };
        assert_eq!(file_not_found.exit_code(), 2);

        let invalid_width = CliError::InvalidWidth { width: 300 };
        assert_eq!(invalid_width.exit_code(), 22);
    }
}
