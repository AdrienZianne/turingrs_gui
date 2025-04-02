use egui::{text::LayoutJob, Color32, FontDefinitions, FontId, Ui};

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

    pub const SELECTED: Color32 = Color32::CYAN;

    // Fonts
    pub const BIG_TEXT_SIZE: f32 = 20.0;
    pub const TEXT_SIZE: f32 = 16.0;
    pub const SMALL_TEXT_SIZE: f32 = 12.0;

    pub fn default_font() -> FontId {
        FontId {
            family: egui::FontFamily::Name("RobotoMono-regular".into()),
            size: Constant::TEXT_SIZE,
        }
    }

    pub fn mono_font() -> FontId {
        FontId {
            family: egui::FontFamily::Name("RobotoMono-regular".into()),
            size: Constant::TEXT_SIZE,
        }
    }

    pub fn code_font() -> FontId {
        FontId {
            family: egui::FontFamily::Name("Roboto".into()),
            size: Constant::TEXT_SIZE,
        }
    }
    pub fn small_font() -> FontId {
        FontId {
            family: egui::FontFamily::Name("Roboto".into()),
            size: Constant::SMALL_TEXT_SIZE,
        }
    }
    pub fn big_font() -> FontId {
        FontId {
            family: egui::FontFamily::Name("Roboto".into()),
            size: Constant::BIG_TEXT_SIZE,
        }
    }

    pub fn get_heigt(ui: &Ui, fond_id: &FontId) -> f32 {
        ui.fonts(|f| {
            f.row_height(fond_id)
        })
    }

    pub fn get_width(ui: &Ui, fond_id: &FontId) -> f32 {
        ui.fonts(|f| {
            f.glyph_width(fond_id, 'M')
        })
    }

    // Graph
    pub const CREP: f32 = 10000.0;
    pub const CSPRING: f32 = 50.0;
    pub const L: f32 = 250.0;
    pub const MAX_FORCE: f32 = 100.0;
    pub const STATE_RADIUS: f32 = 40.0;
    pub const TRANSITION_THICKNESS: f32 = 1.0;
    pub const STABILITY_TRESHOLD: f32 = 0.01;
    pub const ARROW_SIZE: f32 = 20.0;
    pub const TRANSITION_CURVATURE: f32 = 20.0;

    // Ribbon
    pub const SQUARE_SIZE: f32 = 30.0;
    pub const VERTICAL_SPACE: f32 = 8.0;
    pub const HORIZONTAL_SPACE: f32 = 5.0;




}