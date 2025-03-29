use egui::{vec2, Button, Color32, Image, ImageSource, Label, Margin, RichText, Stroke, Style, TextEdit, Ui};

use crate::TuringApp;

use super::constant::Constant;

/// A basic button with text
pub fn button<'a>(style: &mut Style, text: &str) -> Button<'a> {

    style.spacing.button_padding = vec2(10.0, 5.0);

    Button::new(
        RichText::new(text)
        .font(Constant::code_font())
        .color(Constant::PRIMARY_COLOR)
    )
    .stroke(Stroke::new(1.0, Constant::BORDER))
    .fill(Constant::BACKGROUND)
    .corner_radius(10.0)
}

/// A basic button with an image
pub fn button_image<'a>(style: &mut Style, source: ImageSource<'a>) -> Button<'a> {

    style.spacing.button_padding = vec2(10.0, 5.0);

    Button::image(Image::new(source).fit_to_exact_size((24.0, 24.0).into()))
        .stroke(Stroke::new(1.0, Constant::BORDER))
        .fill(Constant::BACKGROUND)
        .corner_radius(10.0)
}

/// A basic label
pub fn label(style: &mut Style, text: &str) -> Label {
    Label::new(
        RichText::new(text)
            .font(Constant::code_font())
            .color(Constant::PRIMARY_COLOR),
    )
}

// A basic label with custom colour
pub fn label_colored(style: &mut Style, text: &str, color: Color32) -> Label {
    Label::new(
        RichText::new(text)
            .font(Constant::code_font())
            .color(color),
    )
}

pub fn text_edit_single<'a>(style: &mut Style, text: &'a mut String) -> TextEdit<'a> {
    style.visuals.clip_rect_margin = 1.0;
    TextEdit::singleline(text)
        .clip_text(true)
        .background_color(Constant::BACKGROUND_2)
        .margin(Margin::symmetric(4, 5))
        .vertical_align(egui::Align::Center)
        .font(Constant::code_font())
}