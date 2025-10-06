use crate::config::{ColorTheme, Settings};
use crate::error::{ConfigError, ConfigResult};
use serde::{Deserialize, Serialize};

/// Complete Mark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkConfig {
    /// General settings
    pub settings: Settings,

    /// Color themes
    pub color: ColorTheme,
}

impl MarkConfig {
    /// Parse configuration from TOML string with strict validation
    pub fn from_toml(content: &str) -> ConfigResult<Self> {
        // First parse as a generic value to check structure
        let value: toml::Value = toml::from_str(content).map_err(|e| {
            let (line, col) = if let Some(span) = e.span() {
                (span.start, span.end)
            } else {
                (0, 0)
            };
            ConfigError::TomlParseError {
                message: e.message().to_string(),
                line,
                col,
            }
        })?;

        // Validate structure before deserializing
        Self::validate_structure(&value)?;

        // Now deserialize with validation
        let config: MarkConfig =
            toml::from_str(content).map_err(|e| ConfigError::TomlParseError {
                message: e.message().to_string(),
                line: 0,
                col: 0,
            })?;

        // Additional validation
        config.validate()?;

        Ok(config)
    }

    /// Serialize configuration to TOML string
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    /// Validate the entire configuration structure
    fn validate_structure(value: &toml::Value) -> ConfigResult<()> {
        let table = value
            .as_table()
            .ok_or_else(|| ConfigError::missing_section("root"))?;

        // Check required sections
        if !table.contains_key("settings") {
            return Err(ConfigError::missing_section("settings"));
        }

        if !table.contains_key("color") {
            return Err(ConfigError::missing_section("color"));
        }

        // Validate settings section
        let settings = table["settings"]
            .as_table()
            .ok_or_else(|| ConfigError::missing_section("settings"))?;
        Self::validate_settings_section(settings)?;

        // Validate color section
        let color = table["color"]
            .as_table()
            .ok_or_else(|| ConfigError::missing_section("color"))?;
        Self::validate_color_section(color)?;

        Ok(())
    }

    /// Validate settings section
    fn validate_settings_section(settings: &toml::value::Table) -> ConfigResult<()> {
        let required_fields = vec![
            ("theme", "string"),
            ("width", "integer"),
            ("syntax_highlighting", "boolean"),
            ("hidden_files", "boolean"),
            ("ignored_dirs", "array"),
        ];

        for (field, expected_type) in required_fields {
            if !settings.contains_key(field) {
                return Err(ConfigError::missing_field(field, "settings"));
            }

            let value = &settings[field];
            let is_correct_type = match expected_type {
                "string" => value.is_str(),
                "integer" => value.is_integer(),
                "boolean" => value.is_bool(),
                "array" => value.is_array(),
                _ => false,
            };

            if !is_correct_type {
                return Err(ConfigError::invalid_value(
                    field,
                    "settings",
                    &value.to_string(),
                    expected_type,
                ));
            }
        }

        // Validate theme value
        if let Some(theme) = settings["theme"].as_str() {
            if theme != "dark" && theme != "light" {
                return Err(ConfigError::invalid_theme(theme));
            }
        }

        // Validate width value
        if let Some(width) = settings["width"].as_integer() {
            if width < 20 || width > 200 {
                return Err(ConfigError::invalid_value(
                    "width",
                    "settings",
                    &width.to_string(),
                    "20-200",
                ));
            }
        }

        Ok(())
    }

    /// Validate color section
    fn validate_color_section(color: &toml::value::Table) -> ConfigResult<()> {
        // Check required sub-sections
        if !color.contains_key("dark") {
            return Err(ConfigError::missing_section("color.dark"));
        }

        if !color.contains_key("light") {
            return Err(ConfigError::missing_section("color.light"));
        }

        // Validate dark colors
        let dark = color["dark"]
            .as_table()
            .ok_or_else(|| ConfigError::missing_section("color.dark"))?;
        Self::validate_color_fields(dark, "color.dark")?;

        // Validate light colors
        let light = color["light"]
            .as_table()
            .ok_or_else(|| ConfigError::missing_section("color.light"))?;
        Self::validate_color_fields(light, "color.light")?;

        Ok(())
    }

    /// Validate color fields in a theme
    fn validate_color_fields(colors: &toml::value::Table, section: &str) -> ConfigResult<()> {
        let required_color_fields = vec![
            "background",
            "text",
            "code_block",
            "h1",
            "h2",
            "h3",
            "h4",
            "h5",
            "h6",
            "link",
            "passive",
        ];

        for field in required_color_fields {
            if !colors.contains_key(field) {
                return Err(ConfigError::missing_field(field, section));
            }

            let color_value = colors[field].as_str().ok_or_else(|| {
                ConfigError::invalid_value(
                    field,
                    section,
                    &colors[field].to_string(),
                    "string (hex color)",
                )
            })?;

            Self::validate_hex_color(color_value, field)?;
        }

        Ok(())
    }

    /// Validate hex color format
    fn validate_hex_color(color: &str, field: &str) -> ConfigResult<()> {
        if !color.starts_with('#') {
            return Err(ConfigError::invalid_color(color, field));
        }

        let hex_part = &color[1..];
        if hex_part.len() != 6 {
            return Err(ConfigError::invalid_color(color, field));
        }

        if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ConfigError::invalid_color(color, field));
        }

        Ok(())
    }

    /// Additional validation after deserialization
    pub fn validate(&self) -> ConfigResult<()> {
        // Validate settings
        self.settings.validate()?;

        // Validate colors
        self.color.validate()?;

        Ok(())
    }

    /// Get current theme colors based on settings
    pub fn current_colors(&self) -> Result<&dyn std::fmt::Debug, ConfigError> {
        match self.settings.theme.as_str() {
            "dark" => Ok(&self.color.dark),
            "light" => Ok(&self.color.light),
            theme => Err(ConfigError::invalid_theme(theme)),
        }
    }

    /// List all missing or invalid fields in a config
    pub fn validate_and_collect_errors(content: &str) -> Vec<ConfigError> {
        let mut errors = Vec::new();

        // Try to parse as TOML first
        let value = match toml::from_str::<toml::Value>(content) {
            Ok(v) => v,
            Err(e) => {
                let (line, col) = if let Some(span) = e.span() {
                    (span.start, span.end)
                } else {
                    (0, 0)
                };
                errors.push(ConfigError::TomlParseError {
                    message: e.message().to_string(),
                    line,
                    col,
                });
                return errors;
            }
        };

        // Validate structure and collect all errors
        if let Err(e) = Self::validate_structure(&value) {
            errors.push(e);
        }

        // Try to parse and validate the config itself
        if let Ok(config) = toml::from_str::<MarkConfig>(content) {
            if let Err(e) = config.validate() {
                errors.push(e);
            }
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_sections() {
        let incomplete_config = "
        [settings]
        theme = \"dark\"
        ";

        let errors = MarkConfig::validate_and_collect_errors(incomplete_config);
        assert!(!errors.is_empty());
        assert!(errors
            .iter()
            .any(|e| matches!(e, ConfigError::MissingSection { section } if section == "color")));
    }

    #[test]
    fn test_invalid_theme() {
        let invalid_config = r##"
        [settings]
        theme = "invalid"
        width = 80
        syntax_highlighting = true
        hidden_files = true
        ignored_dirs = []

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

        let result = MarkConfig::from_toml(invalid_config);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigError::InvalidTheme { .. }
        ));
    }

    #[test]
    fn test_invalid_color_format() {
        let invalid_config = r##"
        [settings]
        theme = "dark"
        width = 80
        syntax_highlighting = true
        hidden_files = true
        ignored_dirs = ["node_modules", "go"]

        [color.dark]
        background = "not-a-color"
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

        let result = MarkConfig::from_toml(invalid_config);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConfigError::InvalidColor { .. }
        ));
    }

    #[test]
    fn test_valid_config() {
        let valid_config = r##"
        [settings]
        theme = "dark"
        width = 80
        syntax_highlighting = true
        hidden_files = false
        ignored_dirs = []

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

        let result = MarkConfig::from_toml(valid_config);
        assert!(result.is_ok());
    }
}
