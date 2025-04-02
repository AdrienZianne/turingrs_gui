use constant::Constant;
use egui::{CentralPanel, Frame, Id, Margin, SidePanel, TopBottomPanel};

use crate::TuringApp;

/// ui module import
mod ribbon;
mod code;
mod control;
mod utils;
mod component;
mod graph;
pub mod constant;
pub mod turing;


pub fn show(app: &mut TuringApp, ctx: &egui::Context) {
    
    // Main panel, take all available space
    CentralPanel::default()
    .frame(Frame {
        outer_margin: Margin::same(0),
        inner_margin: Margin::same(10),
        fill: Constant::BACKGROUND,
        ..Default::default()
    })
    .show(ctx, |ui| {

        // Top panel with the ribbons and the controls of the execution
        TopBottomPanel::top(Id::new("Top"))
        .frame(Frame {
            ..Default::default()
        })
        .show_inside(ui, |ui| {
            ui.style_mut().spacing.item_spacing = (10.0,10.0).into();
            ribbon::show(app, ui);
            control::show(app, ui);
        });


        // The remainder space available for graph and code section
        CentralPanel::default()
        .frame(Frame {
            outer_margin: Margin {
                top: 10,
                ..Default::default()
            },
            ..Default::default()
        })
        .show_inside(ui, |ui| {

            // Graph pannel, resizable
            SidePanel::left(Id::new("Graph"))
            .default_width(ui.available_width()/2.0)
            .frame(Frame {
                fill: Constant::BACKGROUND_2,
                inner_margin: Margin::same(0),
                ..Default::default()
            })
            .show_inside(ui, |ui| {
                graph::show(app, ui);
            });

            // Code pannel, taking the space remaining
            CentralPanel::default()
            .frame(Frame {
                ..Default::default()
            })
            .show_inside(ui, |ui| {
                code::show(app, ui);
            }); 
            
        });

    });
}