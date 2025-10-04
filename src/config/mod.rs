pub mod colors;
pub mod loader;
pub mod parser;
pub mod settings;

use std::path::PathBuf;

// Re-export main types
pub use colors::ColorTheme;
pub use loader::ConfigLoader;
pub use parser::MarkConfig;
pub use settings::Settings;

use crate::error::MarkError;

pub fn get_default_config_path() -> std::result::Result<PathBuf, MarkError> {
    let home = std::env::var("HOME")
        .map_err(|_| crate::error::MarkError::config("HOME environment variable not set"))?;

    Ok(std::path::PathBuf::from(home)
        .join(".config")
        .join("mark")
        .join("config.toml"))
}
