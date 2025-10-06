use crate::error::{ConfigError, ConfigResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: String,
    pub width: usize,
    pub syntax_highlighting: bool,
    pub hidden_files: bool,
    pub ignored_dirs: Vec<String>,
}

impl Settings {
    /// Validate the settings configuration
    pub fn validate(&self) -> ConfigResult<()> {
        // Validate theme
        if self.theme != "dark" && self.theme != "light" {
            return Err(ConfigError::invalid_theme(self.theme.as_str()));
        }

        // Validate width
        if self.width < 20 || self.width > 200 {
            return Err(ConfigError::invalid_value(
                "width",
                "settings",
                &self.width.to_string(),
                "20-200",
            ));
        }

        Ok(())
    }

    /// Check if using dark theme
    pub fn is_dark_theme(&self) -> bool {
        self.theme == "dark"
    }

    /// Check if using light theme
    pub fn is_light_theme(&self) -> bool {
        self.theme == "light"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_settings() {
        let settings = Settings {
            theme: "dark".to_string(),
            width: 80,
            syntax_highlighting: true,
            hidden_files: false,
            ignored_dirs: vec!["node_modules".to_string(), "go".to_string()],
        };

        assert!(settings.validate().is_ok());
        assert!(settings.is_dark_theme());
        assert!(!settings.is_light_theme());
    }

    #[test]
    fn test_invalid_theme() {
        let settings = Settings {
            theme: "invalid".to_string(),
            width: 80,
            syntax_highlighting: true,
            hidden_files: false,
            ignored_dirs: vec![],
        };

        let result = settings.validate();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigError::InvalidTheme { .. }
        ));
    }

    #[test]
    fn test_invalid_width() {
        let settings = Settings {
            theme: "dark".to_string(),
            width: 300, // Too large
            syntax_highlighting: true,
            hidden_files: false,
            ignored_dirs: vec![],
        };

        let result = settings.validate();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigError::InvalidValue { .. }
        ));
    }

    #[test]
    fn test_theme_helpers() {
        let dark_settings = Settings {
            theme: "dark".to_string(),
            width: 80,
            syntax_highlighting: true,
            hidden_files: false,
            ignored_dirs: vec![],
        };

        let light_settings = Settings {
            theme: "light".to_string(),
            width: 80,
            syntax_highlighting: true,
            hidden_files: false,
            ignored_dirs: vec![],
        };

        assert!(dark_settings.is_dark_theme());
        assert!(!dark_settings.is_light_theme());

        assert!(!light_settings.is_dark_theme());
        assert!(light_settings.is_light_theme());
    }
}
