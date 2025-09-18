use eframe::egui::{Color32, Rgba};

/// Color scheme constants for the Fade Launcher
/// Features a beautiful gradient from pink to aqua blue
pub struct FadeTheme;

impl FadeTheme {
    // Primary gradient colors
    pub const PINK_PRIMARY: Color32 = Color32::from_rgb(255, 105, 180);     // Hot pink
    pub const PINK_SECONDARY: Color32 = Color32::from_rgb(255, 182, 193);   // Light pink
    pub const AQUA_PRIMARY: Color32 = Color32::from_rgb(0, 255, 255);       // Cyan/Aqua
    pub const AQUA_SECONDARY: Color32 = Color32::from_rgb(127, 255, 212);   // Aquamarine
    
    // Gradient transition colors
    pub const GRADIENT_MID: Color32 = Color32::from_rgb(127, 180, 217);     // Purple-blue mix
    pub const GRADIENT_LIGHT: Color32 = Color32::from_rgb(200, 162, 235);   // Light lavender
    
    // UI element colors
    pub const BACKGROUND_DARK: Color32 = Color32::from_rgba_premultiplied(20, 20, 30, 240);
    pub const BACKGROUND_LIGHT: Color32 = Color32::from_rgba_premultiplied(40, 40, 60, 200);
    pub const TEXT_PRIMARY: Color32 = Color32::WHITE;
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(200, 200, 200);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(160, 160, 160);
    
    // Interactive elements
    pub const HOVER_COLOR: Color32 = Color32::from_rgba_premultiplied(255, 105, 180, 100);
    pub const ACTIVE_COLOR: Color32 = Color32::from_rgba_premultiplied(0, 255, 255, 150);
    pub const BORDER_COLOR: Color32 = Color32::from_rgba_premultiplied(127, 180, 217, 180);
    
    // Search box colors
    pub const SEARCH_BACKGROUND: Color32 = Color32::from_rgba_premultiplied(30, 30, 45, 220);
    pub const SEARCH_BORDER: Color32 = Color32::from_rgba_premultiplied(127, 180, 217, 255);
    pub const SEARCH_FOCUS: Color32 = Color32::from_rgba_premultiplied(255, 105, 180, 255);
}

impl FadeTheme {
    /// Creates a gradient color at position t (0.0 to 1.0) between pink and aqua
    pub fn gradient_color(t: f32) -> Color32 {
        let t = t.clamp(0.0, 1.0);
        
        // Interpolate between pink and aqua through purple
        if t < 0.5 {
            // Pink to purple transition
            let local_t = t * 2.0;
            Self::lerp_color(Self::PINK_PRIMARY, Self::GRADIENT_MID, local_t)
        } else {
            // Purple to aqua transition
            let local_t = (t - 0.5) * 2.0;
            Self::lerp_color(Self::GRADIENT_MID, Self::AQUA_PRIMARY, local_t)
        }
    }
    
    /// Linear interpolation between two colors
    fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
        let t = t.clamp(0.0, 1.0);
        Color32::from_rgb(
            (a.r() as f32 * (1.0 - t) + b.r() as f32 * t) as u8,
            (a.g() as f32 * (1.0 - t) + b.g() as f32 * t) as u8,
            (a.b() as f32 * (1.0 - t) + b.b() as f32 * t) as u8,
        )
    }
    
    /// Get a semi-transparent version of a color
    pub fn with_alpha(color: Color32, alpha: u8) -> Color32 {
        Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), alpha)
    }
    
    /// Creates a subtle glow effect color
    pub fn glow_color(base_color: Color32, intensity: f32) -> Color32 {
        let intensity = intensity.clamp(0.0, 1.0);
        Self::with_alpha(base_color, (255.0 * intensity * 0.3) as u8)
    }
}