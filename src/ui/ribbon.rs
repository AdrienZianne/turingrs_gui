use egui::{
    scroll_area::ScrollBarVisibility, vec2, Color32, CornerRadius, Frame, Label, Margin, Rect, RichText, ScrollArea, Sense, Stroke, Ui
};
use unicode_segmentation::UnicodeSegmentation;

use crate::TuringApp;

use super::constant::Constant;

pub fn show(app: &mut TuringApp, ui: &mut Ui) {
    let ribbons_count: u8 = app.turing.turing_machine.k+1;

    // Frame of the ribbons
    let ribbon_frame = Frame::new()
        .inner_margin(Margin::same(10))
        .outer_margin(Margin::same(1))
        .corner_radius(CornerRadius::same(5))
        .stroke(Stroke::new(1.0, Constant::BORDER))
        .show(ui, |ui| {
            // Vertical box for ribbons
            ui.vertical(|ui| {
                // Spacing between the item
                ui.spacing_mut().item_spacing = (5.0, 8.0).into();

                let available_width = ui.available_width();

                // Calculate the remaining width available for the ribbons
                // this computation is handmade and can be deobfuscated if you have time
                let square_count = (available_width + Constant::HORIZONTAL_SPACE)
                    / (Constant::SQUARE_SIZE + Constant::HORIZONTAL_SPACE);
                let remaining_width = available_width - 
                    (
                        square_count + 2.0
                        // if even add a square in the calculation
                        + (1 - square_count.floor() as usize % 2) as f32).floor()
                        * (Constant::SQUARE_SIZE + Constant::HORIZONTAL_SPACE)
                    + Constant::HORIZONTAL_SPACE;


                // ScrollArea to make the ribbon "disappear" on the side
                ScrollArea::horizontal()
                    .enable_scrolling(false)
                    .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                    .horizontal_scroll_offset(-remaining_width / 2.0)
                    .show(ui, |ui| {
                        // Draw each ribbons
                        for i in 0..ribbons_count {
                            ruban(app, ui, i.into(), available_width);
                        }
                    })
            });
        });

    ui.painter().rect_stroke(
        Rect::from_center_size(
            ribbon_frame.response.rect.center(),
            vec2(
                Constant::SQUARE_SIZE,
                (ribbons_count * 30 + (ribbons_count - 1) * 5) as f32),
        ),
        CornerRadius::ZERO,
        Stroke::new(5.0, Constant::BORDER),
        egui::StrokeKind::Outside,
    );
}

fn ruban(app: &mut TuringApp, ui: &mut Ui, index: usize, available_width: f32) {

    // Horizontal box for each square
    ui.horizontal(|ui| {
        // Compute the square count
        let mut square_count =
            ((available_width + Constant::HORIZONTAL_SPACE) / (Constant::SQUARE_SIZE + Constant::HORIZONTAL_SPACE)) + 2.0;
        
        square_count += if square_count as usize % 2 == 0 {
            1.0
        } else {
            0.0
        };

        // fetch the current step
        let current_step = &app.current_step;

        // p is the current pointer of the ribbon
        // input is the current word in the ribbon
        let p: i32;
        let input: String;
        if index == 0 {
            p = (square_count as i32 / 2) - current_step.read_ribbon.pointer as i32;
            input = current_step.read_ribbon.chars_vec.iter().collect();
        } else {
            p = (square_count as i32 / 2) - current_step.write_ribbons[index - 1].pointer as i32;
            input = current_step.write_ribbons[index - 1]
                .chars_vec
                .iter()
                .collect();
        }

        for i in 0..square_count as usize {
            if p <= i as i32 && i as i32 - p < input.graphemes(true).count() as i32 {
                draw_square(ui, input.chars().nth((i as i32 - p) as usize).unwrap());
            } else {
                draw_square(ui, ' ');
            }
        }
    });
}

/// Draw a square of the rubban with the character specified
fn draw_square(ui: &mut Ui, t: char) {
    Frame::new().fill(Constant::FOREGROUND).show(ui, |ui| {
        let (rect, _res) = ui.allocate_exact_size((30.0, 30.0).into(), Sense::empty());

        ui.put(
            rect,
            Label::new(
                RichText::new(t)
                    .font(Constant::ribbon_font())
                    .color(Color32::WHITE),
            ),
        );
    });
}
