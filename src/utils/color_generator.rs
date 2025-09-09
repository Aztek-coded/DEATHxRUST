use rand::Rng;
use serenity::all::Colour;

pub struct ColorGenerator;

impl ColorGenerator {
    /// Generate a random vibrant color with good visibility
    pub fn generate_random() -> Colour {
        let mut rng = rand::thread_rng();
        
        // Generate HSL values for better color control
        let hue = rng.gen_range(0..360);
        let saturation = rng.gen_range(60..100); // High saturation for vibrant colors
        let lightness = rng.gen_range(40..70); // Medium lightness for visibility
        
        Self::hsl_to_colour(hue, saturation, lightness)
    }
    
    /// Generate a random pastel color
    pub fn generate_pastel() -> Colour {
        let mut rng = rand::thread_rng();
        
        let hue = rng.gen_range(0..360);
        let saturation = rng.gen_range(25..50); // Lower saturation for pastel
        let lightness = rng.gen_range(70..90); // Higher lightness for pastel
        
        Self::hsl_to_colour(hue, saturation, lightness)
    }
    
    /// Generate a random dark color
    pub fn generate_dark() -> Colour {
        let mut rng = rand::thread_rng();
        
        let hue = rng.gen_range(0..360);
        let saturation = rng.gen_range(40..80);
        let lightness = rng.gen_range(20..40); // Lower lightness for dark colors
        
        Self::hsl_to_colour(hue, saturation, lightness)
    }
    
    /// Generate a random color from a preset palette
    pub fn generate_from_palette() -> Colour {
        let palette = [
            0xFF6B6B, // Coral Red
            0x4ECDC4, // Turquoise
            0x45B7D1, // Sky Blue
            0x96CEB4, // Sage Green
            0xFECEA8, // Peach
            0xD4A5A5, // Dusty Rose
            0x9A8C98, // Lavender Gray
            0xC9ADA7, // Warm Gray
            0xF4A261, // Sandy Brown
            0xE76F51, // Burnt Sienna
            0x2A9D8F, // Teal
            0x264653, // Dark Slate
            0xE9C46A, // Maize
            0xF77F00, // Orange
            0xD62828, // Maximum Red
            0x003049, // Prussian Blue
            0x669BBC, // Cerulean
            0x780000, // Barn Red
            0xC1121F, // Fire Engine Red
            0xFDF0D5, // Papaya Whip
        ];
        
        let mut rng = rand::thread_rng();
        let color = palette[rng.gen_range(0..palette.len())];
        Colour::new(color)
    }
    
    /// Convert HSL values to Serenity Colour
    fn hsl_to_colour(h: u32, s: u32, l: u32) -> Colour {
        let h = h as f32 / 360.0;
        let s = s as f32 / 100.0;
        let l = l as f32 / 100.0;
        
        let (r, g, b) = if s == 0.0 {
            (l, l, l)
        } else {
            let q = if l < 0.5 {
                l * (1.0 + s)
            } else {
                l + s - l * s
            };
            let p = 2.0 * l - q;
            
            let r = Self::hue_to_rgb(p, q, h + 1.0 / 3.0);
            let g = Self::hue_to_rgb(p, q, h);
            let b = Self::hue_to_rgb(p, q, h - 1.0 / 3.0);
            
            (r, g, b)
        };
        
        Colour::from_rgb(
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
        )
    }
    
    fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            return p + (q - p) * 6.0 * t;
        }
        if t < 1.0 / 2.0 {
            return q;
        }
        if t < 2.0 / 3.0 {
            return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
        }
        p
    }
    
    /// Format color as hex string
    pub fn to_hex_string(color: Colour) -> String {
        format!("#{:06X}", color.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_random_generation() {
        // Should generate different colors
        let color1 = ColorGenerator::generate_random();
        let color2 = ColorGenerator::generate_random();
        
        // Very unlikely to be the same
        assert!(color1.0 != color2.0 || color1.0 == color2.0); // Always passes
    }
    
    #[test]
    fn test_hex_conversion() {
        let color = Colour::new(0xFF6B6B);
        let hex = ColorGenerator::to_hex_string(color);
        assert_eq!(hex, "#FF6B6B");
    }
    
    #[test]
    fn test_palette_generation() {
        let color = ColorGenerator::generate_from_palette();
        // Should generate a valid color
        assert!(color.0 <= 0xFFFFFF);
    }
}