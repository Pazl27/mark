use std::fs;

use std::path::{Path, PathBuf};
use crate::config::MarkConfig;
use crate::error::{ConfigError, ConfigResult, MarkError, Result};

#[cfg(not(test))]
const GITHUB_REPO: &str = "Pazl27/mark";
// TODO: Change to correct file
#[cfg(not(test))]
const CONFIG_FILE_PATH: &str = "config/default.toml";
#[cfg(not(test))]
const GITHUB_RAW_URL: &str = "https://raw.githubusercontent.com";
// TODO: Change to correct file
#[cfg(not(test))]
const DOCUMENTATION_URL: &str = "https://github.com/Pazl27/mark/blob/main/docs/configuration.md";

pub struct ConfigLoader {
    config_path: PathBuf,
    config: Option<MarkConfig>,
}

impl ConfigLoader {
    /// Create config loader with custom path
    pub fn with_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut loader = Self {
            config_path: path.as_ref().to_path_buf(),
            config: None,
        };

        loader.load_config()?;
        Ok(loader)
    }

    /// Load configuration from file
    fn load_config(&mut self) -> Result<()> {
        if !self.config_path.exists() {
            return self.handle_missing_config();
        }

        match self.try_load_config() {
            Ok(config) => {
                self.config = Some(config);
                Ok(())
            }
            Err(e) => {
                self.handle_invalid_config(&e)?;
                Err(MarkError::ConfigError(e))
            }
        }
    }

    /// Try to load and parse existing config file
    fn try_load_config(&self) -> ConfigResult<MarkConfig> {
        let content = fs::read_to_string(&self.config_path)
            .map_err(|_| ConfigError::FileNotFound {
                path: self.config_path.clone(),
            })?;

        MarkConfig::from_toml(&content)
    }

    /// Handle missing configuration file
    fn handle_missing_config(&mut self) -> Result<()> {
        #[cfg(test)]
        {
            return Err(MarkError::ConfigError(ConfigError::FileNotFound {
                path: self.config_path.clone(),
            }));
        }

        #[cfg(not(test))]
        {
            eprintln!("Configuration file not found: {}", self.config_path.display());
            eprintln!();
            eprintln!("Would you like to download the default configuration? [Y/n]");

            if self.prompt_yes_no()? {
                self.download_default_config()?;
                self.load_config()?;
            } else {
                eprintln!();
                eprintln!("Please create a configuration file at: {}", self.config_path.display());
                eprintln!("Documentation: {}", DOCUMENTATION_URL);
                std::process::exit(1);
            }
        }

        Ok(())
    }

    /// Handle invalid configuration file
    fn handle_invalid_config(&self, _error: &ConfigError) -> Result<()> {
        #[cfg(test)]
        {
            return Ok(()); // Don't exit in test mode
        }

        #[cfg(not(test))]
        {
            match _error {
                ConfigError::TomlParseError { message, line, col } => {
                    eprintln!("Configuration parse error at line {}, column {}: {}", line, col, message);
                }
                ConfigError::MissingField { field, section } => {
                    eprintln!("Missing required field '{}' in section [{}]", field, section);
                }
                ConfigError::MissingSection { section } => {
                    eprintln!("Missing required section [{}]", section);
                }
                ConfigError::InvalidValue { field, section, value, expected } => {
                    eprintln!("Invalid value '{}' for field '{}' in section [{}]. Expected: {}", value, field, section, expected);
                }
                ConfigError::InvalidColor { color, field } => {
                    eprintln!("Invalid color '{}' for field '{}'. Expected hex format like '#ffffff'", color, field);
                }
                ConfigError::InvalidTheme { theme } => {
                    eprintln!("Invalid theme '{}'. Must be 'dark' or 'light'", theme);
                }
                _ => {
                    eprintln!("Configuration error: {}", _error);
                }
            }

            eprintln!();
            eprintln!("Configuration file: {}", self.config_path.display());
            eprintln!("Documentation: {}", DOCUMENTATION_URL);

            std::process::exit(78);
        }

        Ok(())
    }

    /// Prompt user for yes/no answer
    #[cfg(not(test))]
    fn prompt_yes_no(&self) -> Result<bool> {
        use std::io::{self, Write};
        io::stdout().flush().map_err(MarkError::Io)?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(MarkError::Io)?;

        let response = input.trim().to_lowercase();
        Ok(response.is_empty() || response == "y" || response == "yes")
    }

    /// Download default configuration from GitHub
    #[cfg(not(test))]
    fn download_default_config(&self) -> Result<()> {
        let url = format!("{}/{}/main/{}",
            GITHUB_RAW_URL, GITHUB_REPO, CONFIG_FILE_PATH);

        eprintln!("Downloading default configuration...");

        // Use blocking client
        let response = reqwest::blocking::get(&url)
            .map_err(|e| MarkError::network(format!("Failed to download config: {}", e)))?;

        if !response.status().is_success() {
            return Err(MarkError::network(format!("HTTP {}: Failed to download config", response.status())));
        }

        let content = response.text()
            .map_err(|e| MarkError::network(format!("Failed to read response: {}", e)))?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                MarkError::network(format!("Failed to create config directory: {}", e))
            })?;
        }

        // Write config file
        fs::write(&self.config_path, content).map_err(|e| {
            MarkError::network(format!("Failed to write config file: {}", e))
        })?;

        eprintln!("Configuration downloaded to: {}", self.config_path.display());

        Ok(())
    }

    /// Get loaded configuration
    pub fn config(&self) -> &MarkConfig {
        self.config.as_ref().expect("Configuration not loaded")
    }

    /// Check if configuration is loaded
    pub fn is_loaded(&self) -> bool {
        self.config.is_some()
    }
}

/// Get default configuration file path
pub fn get_default_config_path() -> Result<PathBuf> {
    let home = std::env::var("HOME")
        .map_err(|_| MarkError::config("HOME environment variable not set"))?;

    Ok(PathBuf::from(home)
        .join(".config")
        .join("mark")
        .join("config.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_config_path_generation() {
        let path = get_default_config_path();
        assert!(path.is_ok());
        let path = path.unwrap();
        assert!(path.to_string_lossy().contains(".config/mark/config.toml"));
    }

    #[test]
    fn test_missing_config_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("nonexistent.toml");

        let result = ConfigLoader::with_path(&config_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_config_loading() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("valid_config.toml");

        let valid_config = r##"
[settings]
theme = "dark"
width = 80
syntax_highlighting = true
line_numbers = false

[color.dark]
background = "#000000"
text = "#ffffff"
code_block = "#333333"
h1 = "#ff0000"
h2 = "#ff0000"
h3 = "#ff0000"
h4 = "#ff0000"
h5 = "#ff0000"
h6 = "#ff0000"
link = "#0000ff"
passive = "#888888"

[color.light]
background = "#ffffff"
text = "#000000"
code_block = "#f0f0f0"
h1 = "#ff0000"
h2 = "#ff0000"
h3 = "#ff0000"
h4 = "#ff0000"
h5 = "#ff0000"
h6 = "#ff0000"
link = "#0000ff"
passive = "#888888"
"##;

        fs::write(&config_path, valid_config).unwrap();

        let loader = ConfigLoader::with_path(&config_path).unwrap();
        assert!(loader.is_loaded());
        assert_eq!(loader.config().settings.theme, "dark");
    }

    #[test]
    fn test_invalid_config_handling() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("invalid_config.toml");

        let invalid_config = r##"
[settings
theme = "dark"
"##;

        fs::write(&config_path, invalid_config).unwrap();

        let result = ConfigLoader::with_path(&config_path);
        assert!(result.is_err());
    }
}
