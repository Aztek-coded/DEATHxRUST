use crate::utils::error::BotError;
use std::collections::HashMap;

pub struct ColorParser;

impl ColorParser {
    pub fn parse(input: &str) -> Result<u32, BotError> {
        let input = input.trim().to_lowercase();

        tracing::debug!("Attempting to parse color: {}", input);

        // Try hex color first
        if let Ok(color) = Self::parse_hex(&input) {
            tracing::debug!("Successfully parsed hex color: {} -> {:#x}", input, color);
            return Ok(color);
        }

        // Try named color
        if let Ok(color) = Self::parse_named(&input) {
            tracing::debug!("Successfully parsed named color: {} -> {:#x}", input, color);
            return Ok(color);
        }

        tracing::warn!("Invalid color: {}", input);
        Err(BotError::InvalidColor(input.to_string()))
    }

    fn parse_hex(input: &str) -> Result<u32, BotError> {
        let hex = if input.starts_with('#') {
            &input[1..]
        } else if input.starts_with("0x") {
            &input[2..]
        } else if input.len() == 6 || input.len() == 3 {
            input
        } else {
            return Err(BotError::InvalidColor(input.to_string()));
        };

        // Handle short hex (3 chars -> 6 chars)
        let hex = if hex.len() == 3 {
            format!(
                "{0}{0}{1}{1}{2}{2}",
                hex.chars().nth(0).unwrap(),
                hex.chars().nth(1).unwrap(),
                hex.chars().nth(2).unwrap()
            )
        } else if hex.len() == 6 {
            hex.to_string()
        } else {
            return Err(BotError::InvalidColor(input.to_string()));
        };

        u32::from_str_radix(&hex, 16).map_err(|_| BotError::InvalidColor(input.to_string()))
    }

    fn parse_named(input: &str) -> Result<u32, BotError> {
        let colors = Self::get_color_map();

        colors
            .get(input)
            .copied()
            .ok_or_else(|| BotError::InvalidColor(input.to_string()))
    }

    fn get_color_map() -> HashMap<&'static str, u32> {
        let mut colors = HashMap::new();

        // Basic colors
        colors.insert("red", 0xFF0000);
        colors.insert("green", 0x00FF00);
        colors.insert("blue", 0x0000FF);
        colors.insert("yellow", 0xFFFF00);
        colors.insert("cyan", 0x00FFFF);
        colors.insert("magenta", 0xFF00FF);
        colors.insert("orange", 0xFFA500);
        colors.insert("purple", 0x800080);
        colors.insert("pink", 0xFFC0CB);
        colors.insert("brown", 0xA52A2A);
        colors.insert("black", 0x000000);
        colors.insert("white", 0xFFFFFF);
        colors.insert("gray", 0x808080);
        colors.insert("grey", 0x808080);

        // Discord colors
        colors.insert("blurple", 0x5865F2);
        colors.insert("greyple", 0x99AAB5);
        colors.insert("dark", 0x2C2F33);
        colors.insert("darker", 0x23272A);
        colors.insert("light", 0x99AAB5);
        colors.insert("lighter", 0xECF0F1);

        // Extended colors
        colors.insert("aqua", 0x00FFFF);
        colors.insert("lime", 0x00FF00);
        colors.insert("navy", 0x000080);
        colors.insert("olive", 0x808000);
        colors.insert("maroon", 0x800000);
        colors.insert("silver", 0xC0C0C0);
        colors.insert("teal", 0x008080);
        colors.insert("fuchsia", 0xFF00FF);
        colors.insert("gold", 0xFFD700);
        colors.insert("indigo", 0x4B0082);
        colors.insert("violet", 0x8A2BE2);
        colors.insert("turquoise", 0x40E0D0);
        colors.insert("coral", 0xFF7F50);
        colors.insert("salmon", 0xFA8072);
        colors.insert("crimson", 0xDC143C);
        colors.insert("hotpink", 0xFF69B4);
        colors.insert("deeppink", 0xFF1493);
        colors.insert("lightblue", 0xADD8E6);
        colors.insert("lightgreen", 0x90EE90);
        colors.insert("lightyellow", 0xFFFFE0);
        colors.insert("lightgray", 0xD3D3D3);
        colors.insert("lightgrey", 0xD3D3D3);
        colors.insert("darkred", 0x8B0000);
        colors.insert("darkgreen", 0x006400);
        colors.insert("darkblue", 0x00008B);
        colors.insert("darkorange", 0xFF8C00);
        colors.insert("darkviolet", 0x9400D3);

        colors
    }

    /// Returns a formatted hex string for display (e.g., "#FF0000")
    pub fn to_hex_string(color: u32) -> String {
        format!("#{:06X}", color)
    }

    /// Validates if a color value is within Discord's acceptable range
    pub fn is_valid_discord_color(color: u32) -> bool {
        color <= 0xFFFFFF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_colors() {
        assert_eq!(ColorParser::parse("#FF0000").unwrap(), 0xFF0000);
        assert_eq!(ColorParser::parse("FF0000").unwrap(), 0xFF0000);
        assert_eq!(ColorParser::parse("0xFF0000").unwrap(), 0xFF0000);
        assert_eq!(ColorParser::parse("#F00").unwrap(), 0xFF0000);
        assert_eq!(ColorParser::parse("F00").unwrap(), 0xFF0000);
    }

    #[test]
    fn test_named_colors() {
        assert_eq!(ColorParser::parse("red").unwrap(), 0xFF0000);
        assert_eq!(ColorParser::parse("RED").unwrap(), 0xFF0000);
        assert_eq!(ColorParser::parse("blue").unwrap(), 0x0000FF);
        assert_eq!(ColorParser::parse("blurple").unwrap(), 0x5865F2);
    }

    #[test]
    fn test_invalid_colors() {
        assert!(ColorParser::parse("invalid").is_err());
        assert!(ColorParser::parse("#GGGGGG").is_err());
        assert!(ColorParser::parse("12345").is_err());
    }

    #[test]
    fn test_hex_string_formatting() {
        assert_eq!(ColorParser::to_hex_string(0xFF0000), "#FF0000");
        assert_eq!(ColorParser::to_hex_string(0x000000), "#000000");
        assert_eq!(ColorParser::to_hex_string(0xFFFFFF), "#FFFFFF");
    }

    #[test]
    fn test_discord_color_validation() {
        assert!(ColorParser::is_valid_discord_color(0xFF0000));
        assert!(ColorParser::is_valid_discord_color(0xFFFFFF));
        assert!(ColorParser::is_valid_discord_color(0x000000));
        assert!(!ColorParser::is_valid_discord_color(0x1000000));
    }
}
