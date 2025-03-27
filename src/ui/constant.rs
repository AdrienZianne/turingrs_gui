use egui::{Color32, FontId};

/// Constant use in the application
#[non_exhaustive]
pub struct Constant;

impl Constant {

    // Colors
    pub const BACKGROUND: Color32 = Color32::from_rgb(62, 62, 62);
    pub const BACKGROUND_2: Color32 = Color32::from_rgb(41, 41, 41);
    pub const BORDER: Color32 = Color32::WHITE;
    pub const FOREGROUND: Color32 = Color32::from_rgb(109, 109, 109);

    pub const PRIMARY_COLOR: Color32 = Color32::WHITE;
    pub const SECONDARY_COLOR: Color32 = Color32::DARK_GRAY;

    pub const POSITIVE_COLOR: Color32 = Color32::GREEN;
    pub const NEGATIVE_COLOR: Color32 = Color32::RED;

    // Fonts
    pub const TEXT_SIZE: f32 = 16.0;
    pub const SMALL_TEXT_SIZE: f32 = 12.0;

    pub fn CODE_FONT() -> FontId {
        FontId {
            family: egui::FontFamily::Name("Roboto".into()),
            size: Constant::TEXT_SIZE,
        }
    }
    pub fn SMALL_FONT() -> FontId {
        FontId {
            family: egui::FontFamily::Name("Roboto".into()),
            size: Constant::SMALL_TEXT_SIZE,
        }
    }

    // Graph
    pub const CREP: f32 = 10000.0;
    pub const CSPRING: f32 = 100.0;
    pub const L: f32 = 200.0;
    pub const MAX_FORCE: f32 = 100000.0;

    // Ribbon
    pub const RIBBON_SQUARE_SIZE: f32 = 35.0;
    pub const VERTICAL_SPACE: f32 = 8.0;
    pub const HORIZONTAL_SPACE: f32 = 5.0;




}