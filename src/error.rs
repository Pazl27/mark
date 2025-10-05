use std::path::PathBuf;
use thiserror::Error;

/// Application error types
#[derive(Error, Debug)]
pub enum MarkError {
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

    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),

    #[error("Network error: {message}")]
    Network { message: String },

    #[error("Search error: {message}")]
    Search { message: String },

    #[error("Lexer error: {0}")]
    Lexer(#[from] LexerError),

    #[error("Parser error: {0}")]
    Parser(#[from] ParseError),
}

/// Configuration-specific error types
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file not found at: {path}")]
    FileNotFound { path: PathBuf },

    #[error("Failed to parse TOML: {message} at line {line}, column {col}")]
    TomlParseError {
        message: String,
        line: usize,
        col: usize,
    },

    #[error("Missing required field: '{field}' in section [{section}]")]
    MissingField { field: String, section: String },

    #[error(
        "Invalid value for field '{field}' in section [{section}]: {value}. Expected: {expected}"
    )]
    InvalidValue {
        field: String,
        section: String,
        value: String,
        expected: String,
    },

    #[error("Missing required section: [{section}]")]
    MissingSection { section: String },

    #[error(
        "Invalid color format: '{color}' in field '{field}'. Expected hex format like '#ffffff'"
    )]
    InvalidColor { color: String, field: String },

    #[error("Invalid theme: '{theme}'. Must be 'dark' or 'light'")]
    InvalidTheme { theme: String },

    #[error("Failed to create config directory: {path}")]
    DirectoryCreationFailed { path: PathBuf },

    #[error("Failed to download default config from GitHub: {message}")]
    DownloadFailed { message: String },

    #[error("User declined to download default configuration")]
    DownloadDeclined,
}

/// Parser-specific error types
#[derive(Error, Debug)]
pub enum ParseError {
    #[error(
        "Unexpected token at line {line}, column {column}: expected {expected}, found {found}"
    )]
    UnexpectedToken {
        expected: String,
        found: String,
        line: usize,
        column: usize,
    },

    #[error("Unexpected end of input: expected {expected}")]
    UnexpectedEndOfInput { expected: String },

    #[error(
        "Invalid heading level {level} at line {line}, column {column}: must be between 1 and 6"
    )]
    InvalidHeadingLevel {
        level: u8,
        line: usize,
        column: usize,
    },

    #[error("Malformed link at line {line}, column {column}: {message}")]
    MalformedLink {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Malformed image at line {line}, column {column}: {message}")]
    MalformedImage {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Invalid list structure at line {line}, column {column}: {message}")]
    InvalidList {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Unmatched delimiter '{delimiter}' at line {line}, column {column}")]
    UnmatchedDelimiter {
        delimiter: char,
        line: usize,
        column: usize,
    },

    #[error("Invalid table structure at line {line}, column {column}: {message}")]
    InvalidTable {
        message: String,
        line: usize,
        column: usize,
    },
}

impl From<crate::error::LexerError> for ParseError {
    fn from(lexer_error: crate::error::LexerError) -> Self {
        match lexer_error {
            crate::error::LexerError::UnexpectedCharacter {
                character,
                line,
                column,
            } => ParseError::UnexpectedToken {
                expected: "valid markdown character".to_string(),
                found: character.to_string(),
                line,
                column,
            },
            crate::error::LexerError::UnterminatedCodeBlock { line, column } => {
                ParseError::UnmatchedDelimiter {
                    delimiter: '`',
                    line,
                    column,
                }
            }
            crate::error::LexerError::InvalidSyntax {
                message,
                line,
                column,
            } => ParseError::UnexpectedToken {
                expected: "valid markdown syntax".to_string(),
                found: message,
                line,
                column,
            },
            crate::error::LexerError::InvalidUrl { url, line, column } => {
                ParseError::MalformedLink {
                    message: format!("Invalid URL: {}", url),
                    line,
                    column,
                }
            }
            crate::error::LexerError::NumberTooLarge {
                value,
                line,
                column,
            } => ParseError::UnexpectedToken {
                expected: "valid number".to_string(),
                found: value,
                line,
                column,
            },
        }
    }
}

/// Lexer-specific error types
#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected character '{character}' at line {line}, column {column}")]
    UnexpectedCharacter {
        character: char,
        line: usize,
        column: usize,
    },

    #[error("Unterminated code block starting at line {line}, column {column}")]
    UnterminatedCodeBlock { line: usize, column: usize },

    #[error("Invalid markdown syntax at line {line}, column {column}: {message}")]
    InvalidSyntax {
        message: String,
        line: usize,
        column: usize,
    },

    #[error("Invalid URL format at line {line}, column {column}: {url}")]
    InvalidUrl {
        url: String,
        line: usize,
        column: usize,
    },

    #[error("Number too large at line {line}, column {column}: {value}")]
    NumberTooLarge {
        value: String,
        line: usize,
        column: usize,
    },
}

/// Result type alias for application results
pub type Result<T> = std::result::Result<T, MarkError>;

/// Result type alias for configuration operations
pub type ConfigResult<T> = std::result::Result<T, ConfigError>;

/// Result type alias for lexer operations
pub type LexerResult<T> = std::result::Result<T, LexerError>;

/// Result type alias for parser operations
pub type ParseResult<T> = std::result::Result<T, ParseError>;

impl MarkError {
    /// Create a new configuration error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Create a new network error
    pub fn network<S: Into<String>>(message: S) -> Self {
        Self::Network {
            message: message.into(),
        }
    }

    /// Create a new search error
    pub fn search<S: Into<String>>(message: S) -> Self {
        Self::Search {
            message: message.into(),
        }
    }

    /// Get the exit code for this error
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::FileNotFound { .. } => 2,
            Self::InvalidFileFormat { .. } | Self::InvalidWidth { .. } => 22,
            Self::Config { .. } | Self::ConfigError(_) => 78,
            Self::Network { .. } => 7,
            Self::Search { .. } => 3,
            Self::Lexer(_) => 65,
            Self::Parser(_) => 66,
            Self::Io(_) => 1,
        }
    }
}

impl ConfigError {
    /// Create a missing field error
    pub fn missing_field<S: Into<String>>(field: S, section: S) -> Self {
        Self::MissingField {
            field: field.into(),
            section: section.into(),
        }
    }

    /// Create an invalid value error
    pub fn invalid_value<S: Into<String>>(field: S, section: S, value: S, expected: S) -> Self {
        Self::InvalidValue {
            field: field.into(),
            section: section.into(),
            value: value.into(),
            expected: expected.into(),
        }
    }

    /// Create a missing section error
    pub fn missing_section<S: Into<String>>(section: S) -> Self {
        Self::MissingSection {
            section: section.into(),
        }
    }

    /// Create an invalid color error
    pub fn invalid_color<S: Into<String>>(color: S, field: S) -> Self {
        Self::InvalidColor {
            color: color.into(),
            field: field.into(),
        }
    }

    /// Create an invalid theme error
    pub fn invalid_theme<S: Into<String>>(theme: S) -> Self {
        Self::InvalidTheme {
            theme: theme.into(),
        }
    }

    /// Create a download failed error
    pub fn download_failed<S: Into<String>>(message: S) -> Self {
        Self::DownloadFailed {
            message: message.into(),
        }
    }
}

impl ParseError {
    /// Create an unexpected token error
    pub fn unexpected_token<S: Into<String>>(
        expected: S,
        found: S,
        line: usize,
        column: usize,
    ) -> Self {
        Self::UnexpectedToken {
            expected: expected.into(),
            found: found.into(),
            line,
            column,
        }
    }

    /// Create an unexpected end of input error
    pub fn unexpected_end_of_input<S: Into<String>>(expected: S) -> Self {
        Self::UnexpectedEndOfInput {
            expected: expected.into(),
        }
    }

    /// Create an invalid heading level error
    pub fn invalid_heading_level(level: u8, line: usize, column: usize) -> Self {
        Self::InvalidHeadingLevel {
            level,
            line,
            column,
        }
    }

    /// Create a malformed link error
    pub fn malformed_link<S: Into<String>>(message: S, line: usize, column: usize) -> Self {
        Self::MalformedLink {
            message: message.into(),
            line,
            column,
        }
    }

    /// Create a malformed image error
    pub fn malformed_image<S: Into<String>>(message: S, line: usize, column: usize) -> Self {
        Self::MalformedImage {
            message: message.into(),
            line,
            column,
        }
    }

    /// Create an invalid list error
    pub fn invalid_list<S: Into<String>>(message: S, line: usize, column: usize) -> Self {
        Self::InvalidList {
            message: message.into(),
            line,
            column,
        }
    }

    /// Create an unmatched delimiter error
    pub fn unmatched_delimiter(delimiter: char, line: usize, column: usize) -> Self {
        Self::UnmatchedDelimiter {
            delimiter,
            line,
            column,
        }
    }

    /// Create an invalid table error
    pub fn invalid_table<S: Into<String>>(message: S, line: usize, column: usize) -> Self {
        Self::InvalidTable {
            message: message.into(),
            line,
            column,
        }
    }
}

impl LexerError {
    /// Create an unexpected character error
    pub fn unexpected_character(character: char, line: usize, column: usize) -> Self {
        Self::UnexpectedCharacter {
            character,
            line,
            column,
        }
    }

    /// Create an unterminated code block error
    pub fn unterminated_code_block(line: usize, column: usize) -> Self {
        Self::UnterminatedCodeBlock { line, column }
    }

    /// Create an invalid syntax error
    pub fn invalid_syntax<S: Into<String>>(message: S, line: usize, column: usize) -> Self {
        Self::InvalidSyntax {
            message: message.into(),
            line,
            column,
        }
    }

    /// Create an invalid URL error
    pub fn invalid_url<S: Into<String>>(url: S, line: usize, column: usize) -> Self {
        Self::InvalidUrl {
            url: url.into(),
            line,
            column,
        }
    }

    /// Create a number too large error
    pub fn number_too_large<S: Into<String>>(value: S, line: usize, column: usize) -> Self {
        Self::NumberTooLarge {
            value: value.into(),
            line,
            column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let path = PathBuf::from("test.txt");
        let error = MarkError::FileNotFound { path: path.clone() };
        assert!(error.to_string().contains("test.txt"));
    }

    #[test]
    fn test_config_error() {
        let error = MarkError::config("Invalid theme");
        assert!(error.to_string().contains("Invalid theme"));
    }

    #[test]
    fn test_config_specific_errors() {
        let missing_field = ConfigError::missing_field("theme", "settings");
        assert!(missing_field.to_string().contains("theme"));
        assert!(missing_field.to_string().contains("settings"));

        let invalid_color = ConfigError::invalid_color("#zzzzzz", "background");
        assert!(invalid_color.to_string().contains("#zzzzzz"));
        assert!(invalid_color.to_string().contains("background"));
    }

    #[test]
    fn test_exit_codes() {
        let file_not_found = MarkError::FileNotFound {
            path: PathBuf::from("test.md"),
        };
        assert_eq!(file_not_found.exit_code(), 2);

        let config_error = MarkError::ConfigError(ConfigError::missing_section("settings"));
        assert_eq!(config_error.exit_code(), 78);

        let network_error = MarkError::network("Connection failed");
        assert_eq!(network_error.exit_code(), 7);

        let search_error = MarkError::search("Invalid search pattern");
        assert_eq!(search_error.exit_code(), 3);

        let lexer_error = MarkError::Lexer(LexerError::unexpected_character('$', 1, 5));
        assert_eq!(lexer_error.exit_code(), 65);

        let parser_error = MarkError::Parser(ParseError::unexpected_token("heading", "text", 1, 1));
        assert_eq!(parser_error.exit_code(), 66);
    }

    #[test]
    fn test_lexer_errors() {
        let unexpected_char = LexerError::unexpected_character('$', 1, 5);
        assert!(unexpected_char.to_string().contains("'$'"));
        assert!(unexpected_char.to_string().contains("line 1"));
        assert!(unexpected_char.to_string().contains("column 5"));

        let unterminated = LexerError::unterminated_code_block(2, 10);
        assert!(unterminated.to_string().contains("line 2"));
        assert!(unterminated.to_string().contains("column 10"));

        let invalid_syntax = LexerError::invalid_syntax("Missing closing bracket", 3, 15);
        assert!(invalid_syntax
            .to_string()
            .contains("Missing closing bracket"));
        assert!(invalid_syntax.to_string().contains("line 3"));
    }

    #[test]
    fn test_parser_errors() {
        let unexpected_token = ParseError::unexpected_token("heading", "text", 1, 5);
        assert!(unexpected_token.to_string().contains("expected heading"));
        assert!(unexpected_token.to_string().contains("found text"));
        assert!(unexpected_token.to_string().contains("line 1"));
        assert!(unexpected_token.to_string().contains("column 5"));

        let unexpected_end = ParseError::unexpected_end_of_input("closing bracket");
        assert!(unexpected_end
            .to_string()
            .contains("expected closing bracket"));

        let invalid_heading = ParseError::invalid_heading_level(7, 2, 10);
        assert!(invalid_heading.to_string().contains("level 7"));
        assert!(invalid_heading.to_string().contains("between 1 and 6"));

        let malformed_link = ParseError::malformed_link("Missing URL", 3, 15);
        assert!(malformed_link.to_string().contains("Missing URL"));
        assert!(malformed_link.to_string().contains("line 3"));
    }
}
