use egui::{CentralPanel, Frame, Id, SidePanel, TopBottomPanel, Ui};

use crate::TuringApp;

/// ui module import
mod ribbon;
mod code;
mod control;
mod utils;
mod component;
mod graph;
mod constant;
mod turing;


pub fn show(app: &mut TuringApp, ctx: &egui::Context) {
    
    // Main panel, take all available space
    CentralPanel::default()
    .frame(Frame {
        ..Default::default()
    })
    .show(ctx, |ui| {

        // Top panel with the ribbons and the controls of the execution
        TopBottomPanel::top(Id::new("Top"))
        .frame(Frame {
            ..Default::default()
        })
        .show_inside(ui, |ui| {
            ribbon::show(app, ui);
            control::show(app, ui);
        });


        // The remainder space available for graph and code section
        CentralPanel::default()
        .frame(Frame {
            ..Default::default()
        })
        .show_inside(ui, |ui| {

            // Graph pannel, resizable
            SidePanel::left(Id::new("Graph"))
            .frame(Frame {
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