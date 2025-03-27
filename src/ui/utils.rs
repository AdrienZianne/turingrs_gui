use eframe::egui::{Pos2, Vec2};
use egui::Color32;

use crate::ui::constant::Constant;

// compute the distance between 2 points
pub fn distance(p1: Pos2, p2: Pos2) -> f32 {
    f32::sqrt((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2))
}

// compute the repulsion force of the node
pub fn rep_force(p1: Pos2, p2: Pos2) -> f32 {
    let force = Constant::CREP / distance(p1, p2).powi(2);
    f32::max(-Constant::MAX_FORCE, f32::min(Constant::MAX_FORCE, force))
}

// compute the attraction force of the node
pub fn attract_force(p1: Pos2, p2: Pos2) -> f32 {
    let force = Constant::CSPRING * (distance(p1, p2) / (Constant::L)).log(10.0);
    f32::max(-Constant::MAX_FORCE, f32::min(Constant::MAX_FORCE, force))
}

// compute the direction between 2 points
pub fn direction(p1: Pos2, p2: Pos2) -> Vec2 {
    Vec2::new(p2.x - p1.x, p2.y - p1.y).normalized()
}

// compute the best contrast color
pub fn constrast_color(color: Color32) -> Color32 {

    let luminance = (
        0.299 * color.r() as f32
        + 0.587 * color.g() as f32
        + 0.114 * color.b() as f32
    )/255.0;

    if luminance > 0.5 {
        Color32::BLACK
    } else {
        Color32::WHITE
    }
}