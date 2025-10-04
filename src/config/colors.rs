use serde::{Deserialize, Serialize};
use crate::error::{ConfigError, ConfigResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorTheme {
    pub dark: DarkColors,
    pub light: LightColors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DarkColors {
    pub background: String,
    pub text: String,
    pub code_block: String,
    pub h1: String,
    pub h2: String,
    pub h3: String,
    pub h4: String,
    pub h5: String,
    pub h6: String,
    pub link: String,
    pub passive: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightColors {
    pub background: String,
    pub text: String,
    pub code_block: String,
    pub h1: String,
    pub h2: String,
    pub h3: String,
    pub h4: String,
    pub h5: String,
    pub h6: String,
    pub link: String,
    pub passive: String,
}

impl ColorTheme {
    /// Validate the entire color theme
    pub fn validate(&self) -> ConfigResult<()> {
        self.dark.validate()?;
        self.light.validate()?;
        Ok(())
    }
}

impl DarkColors {
    /// Validate all dark color fields
    pub fn validate(&self) -> ConfigResult<()> {
        let colors = vec![
            ("background", &self.background),
            ("text", &self.text),
            ("code_block", &self.code_block),
            ("h1", &self.h1),
            ("h2", &self.h2),
            ("h3", &self.h3),
            ("h4", &self.h4),
            ("h5", &self.h5),
            ("h6", &self.h6),
            ("link", &self.link),
            ("passive", &self.passive),
        ];

        for (field_name, color_value) in colors {
            validate_hex_color(color_value, field_name)?;
        }

        Ok(())
    }

    /// Get all color fields as a vector for iteration
    pub fn all_colors(&self) -> Vec<(&str, &str)> {
        vec![
            ("background", &self.background),
            ("text", &self.text),
            ("code_block", &self.code_block),
            ("h1", &self.h1),
            ("h2", &self.h2),
            ("h3", &self.h3),
            ("h4", &self.h4),
            ("h5", &self.h5),
            ("h6", &self.h6),
            ("link", &self.link),
            ("passive", &self.passive),
        ]
    }
}

impl LightColors {
    /// Validate all light color fields
    pub fn validate(&self) -> ConfigResult<()> {
        let colors = vec![
            ("background", &self.background),
            ("text", &self.text),
            ("code_block", &self.code_block),
            ("h1", &self.h1),
            ("h2", &self.h2),
            ("h3", &self.h3),
            ("h4", &self.h4),
            ("h5", &self.h5),
            ("h6", &self.h6),
            ("link", &self.link),
            ("passive", &self.passive),
        ];

        for (field_name, color_value) in colors {
            validate_hex_color(color_value, field_name)?;
        }

        Ok(())
    }

    /// Get all color fields as a vector for iteration
    pub fn all_colors(&self) -> Vec<(&str, &str)> {
        vec![
            ("background", &self.background),
            ("text", &self.text),
            ("code_block", &self.code_block),
            ("h1", &self.h1),
            ("h2", &self.h2),
            ("h3", &self.h3),
            ("h4", &self.h4),
            ("h5", &self.h5),
            ("h6", &self.h6),
            ("link", &self.link),
            ("passive", &self.passive),
        ]
    }
}

/// Validate hex color format (#ffffff)
fn validate_hex_color(color: &str, field_name: &str) -> ConfigResult<()> {
    // Check if it starts with #
    if !color.starts_with('#') {
        return Err(ConfigError::invalid_color(color, field_name));
    }

    // Check if it has exactly 7 characters (#xxxxxx)
    if color.len() != 7 {
        return Err(ConfigError::invalid_color(color, field_name));
    }

    // Check if all characters after # are valid hex digits
    let hex_part = &color[1..];
    if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ConfigError::invalid_color(color, field_name));
    }

    Ok(())
}

/// Parse hex color to RGB values
pub fn hex_to_rgb(hex: &str) -> ConfigResult<(u8, u8, u8)> {
    validate_hex_color(hex, "color")?;
    
    let hex_part = &hex[1..];
    let r = u8::from_str_radix(&hex_part[0..2], 16)
        .map_err(|_| ConfigError::invalid_color(hex, "color"))?;
    let g = u8::from_str_radix(&hex_part[2..4], 16)
        .map_err(|_| ConfigError::invalid_color(hex, "color"))?;
    let b = u8::from_str_radix(&hex_part[4..6], 16)
        .map_err(|_| ConfigError::invalid_color(hex, "color"))?;
    
    Ok((r, g, b))
}

/// Convert RGB values to hex color string
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_dark_colors() -> DarkColors {
        DarkColors {
            background: "#000000".to_string(),
            text: "#ffffff".to_string(),
            code_block: "#333333".to_string(),
            h1: "#ff0000".to_string(),
            h2: "#ff4444".to_string(),
            h3: "#ff8888".to_string(),
            h4: "#ffaaaa".to_string(),
            h5: "#ffcccc".to_string(),
            h6: "#ffeeee".to_string(),
            link: "#0000ff".to_string(),
            passive: "#888888".to_string(),
        }
    }

    fn create_valid_light_colors() -> LightColors {
        LightColors {
            background: "#ffffff".to_string(),
            text: "#000000".to_string(),
            code_block: "#f0f0f0".to_string(),
            h1: "#cc0000".to_string(),
            h2: "#aa0000".to_string(),
            h3: "#880000".to_string(),
            h4: "#660000".to_string(),
            h5: "#440000".to_string(),
            h6: "#220000".to_string(),
            link: "#0000cc".to_string(),
            passive: "#666666".to_string(),
        }
    }

    #[test]
    fn test_valid_color_validation() {
        let dark_colors = create_valid_dark_colors();
        assert!(dark_colors.validate().is_ok());

        let light_colors = create_valid_light_colors();
        assert!(light_colors.validate().is_ok());
    }

    #[test]
    fn test_invalid_hex_color_format() {
        assert!(validate_hex_color("not-a-color", "test").is_err());
        assert!(validate_hex_color("#gggggg", "test").is_err());
        assert!(validate_hex_color("#fff", "test").is_err());
        assert!(validate_hex_color("ffffff", "test").is_err());
        assert!(validate_hex_color("#1234567", "test").is_err());
    }

    #[test]
    fn test_valid_hex_color_format() {
        assert!(validate_hex_color("#000000", "test").is_ok());
        assert!(validate_hex_color("#ffffff", "test").is_ok());
        assert!(validate_hex_color("#123abc", "test").is_ok());
        assert!(validate_hex_color("#ABCDEF", "test").is_ok());
    }

    #[test]
    fn test_hex_to_rgb_conversion() {
        assert_eq!(hex_to_rgb("#000000").unwrap(), (0, 0, 0));
        assert_eq!(hex_to_rgb("#ffffff").unwrap(), (255, 255, 255));
        assert_eq!(hex_to_rgb("#ff0000").unwrap(), (255, 0, 0));
        assert_eq!(hex_to_rgb("#00ff00").unwrap(), (0, 255, 0));
        assert_eq!(hex_to_rgb("#0000ff").unwrap(), (0, 0, 255));
    }

    #[test]
    fn test_rgb_to_hex_conversion() {
        assert_eq!(rgb_to_hex(0, 0, 0), "#000000");
        assert_eq!(rgb_to_hex(255, 255, 255), "#ffffff");
        assert_eq!(rgb_to_hex(255, 0, 0), "#ff0000");
        assert_eq!(rgb_to_hex(0, 255, 0), "#00ff00");
        assert_eq!(rgb_to_hex(0, 0, 255), "#0000ff");
    }

    #[test]
    fn test_color_theme_validation() {
        let theme = ColorTheme {
            dark: create_valid_dark_colors(),
            light: create_valid_light_colors(),
        };

        assert!(theme.validate().is_ok());
    }

    #[test]
    fn test_invalid_color_in_theme() {
        let mut dark_colors = create_valid_dark_colors();
        dark_colors.background = "invalid-color".to_string();

        let result = dark_colors.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::InvalidColor { .. }));
    }

    #[test]
    fn test_all_colors_iteration() {
        let dark_colors = create_valid_dark_colors();
        let all_colors = dark_colors.all_colors();
        
        assert_eq!(all_colors.len(), 11);
        assert!(all_colors.iter().any(|(name, _)| *name == "background"));
        assert!(all_colors.iter().any(|(name, _)| *name == "text"));
        assert!(all_colors.iter().any(|(name, _)| *name == "h1"));
    }
}