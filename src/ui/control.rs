use egui::Ui;
use egui_flex::{Flex, FlexAlign, FlexJustify, item};
use turingrs::turing_machine::TuringExecutor;

use crate::TuringApp;

use super::{
    component::{button, button_image, label, label_colored, text_edit_single},
    constant::Constant,
};

/// This module display the controls of the application, including the following :
/// - The word input and button to update it
/// - The play/pause/next/reset buttons
/// - The steps counter and result (Accepted/Rejected)
pub fn show(app: &mut TuringApp, ui: &mut Ui) {
    ui.columns_const(|[left, center, right]| {
        input(app, left);
        control_buttons(app, center);
        result(app, right);
    });
}

fn input(app: &mut TuringApp, ui: &mut Ui) {
    Flex::horizontal()
        .h_full()
        .align_items(FlexAlign::Center)
        .show(ui, |flex| {
            let field = text_edit_single(flex.style_mut(), &mut app.word_input);
            let update = button(flex.style_mut(), "Update");

            flex.add(item().shrink(), field);

            if flex.add(item(), update).clicked() {
                // update word here
            }
        });
}

fn control_buttons(app: &mut TuringApp, ui: &mut Ui) {
    Flex::horizontal()
        .h_full()
        .align_items(FlexAlign::Center)
        .justify(FlexJustify::Center)
        .show(ui, |flex| {
            // initialise the buttons
            let play_button = button_image(
                flex.style_mut(),
                egui::include_image!("../../assets/play.png"),
            );
            let pause_button = button_image(
                flex.style_mut(),
                egui::include_image!("../../assets/pause.png"),
            );
            let next_next = button_image(
                flex.style_mut(),
                egui::include_image!("../../assets/next.png"),
            );
            let reset_button = button_image(
                flex.style_mut(),
                egui::include_image!("../../assets/reset.png"),
            );

            // add action on click
            if flex.add(item(), play_button).clicked() {
                play(app);
            }
            if flex.add(item(), pause_button).clicked() {
                pause(app);
            }
            if flex.add(item(), next_next).clicked() {
                next(app);
            };
            if flex.add(item(), reset_button).clicked() {
                reset(app);
            }
        });
}

fn result(app: &mut TuringApp, ui: &mut Ui) {
    Flex::horizontal()
        .h_full()
        .align_items(FlexAlign::Center)
        .justify(FlexJustify::SpaceAround)
        .show(ui, |flex| {
            let steps = label(flex.style_mut(), &format!("Steps : {}", app.count));
            flex.add(item(), steps);

            let result = match app.is_accepted {
                Some(accepted) => match accepted {
                    true => label_colored(flex.style_mut(), "Accepted", Constant::POSITIVE_COLOR),
                    false => label_colored(flex.style_mut(), "Refused", Constant::NEGATIVE_COLOR),
                },
                None => label(flex.style_mut(), "..."),
            };
            flex.add(item(), result);
        });
}

fn play(app: &mut TuringApp) {}

fn pause(app: &mut TuringApp) {}

/// Fetch the next state from the executor if there is one.
/// if not check if the resulting state is rejecting or accepting
fn next(app: &mut TuringApp) {
    match app.turing.as_iter().next() {
        // Update the current step and increment the counter
        Some(next_step) => {
            app.current_step = next_step;
            app.count += 1;
        }
        // If there is no next step, check if the current state is final.
        // If yes, then it's accepted, if not it's rejected
        None => {
            app.is_accepted = Some(
                app.turing
                    .turing_machine
                    .get_state(app.turing.get_state_pointer())
                    .is_final,
            )
        }
    }
}

/// TODO see how to centralize these methods, maybe a new files or in a new impl in app.rs
fn reset(app: &mut TuringApp) {}
