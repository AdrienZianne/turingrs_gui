use std::collections::BTreeMap;

use egui::{FontData, FontDefinitions, FontFamily};
use egui_extras::install_image_loaders;
use turingrs::turing_machine::{TuringExecutionStep, TuringMachine, TuringMachineExecutor};

use crate::ui;


/// The application data, not refresh after each draw
pub struct TuringApp {
    pub turing: TuringMachineExecutor,
    pub current_step: TuringExecutionStep
}

/// Default implementation of TuringApp
impl Default for TuringApp {
    fn default() -> Self {
        
        // initalize the turing machine crate
        let turing_machine = TuringMachine::new(0);
        let (turing_executor, initial_turing_step) = TuringMachineExecutor::new(
            turing_machine, 
            "".to_string()
        ).expect("Error while creating executor");

        // Implement the TuringApp
        Self {
            turing: turing_executor,
            current_step : initial_turing_step
        }
    }
}

/// Global setting of egui
impl TuringApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        cc.egui_ctx.set_debug_on_hover(true);
        
        load_font(cc);

        Default::default()
    }
}

/// Update loop
impl eframe::App for TuringApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        install_image_loaders(ctx);
        ui::show(self, ctx);
    }
}


/// Load the necessary font for the application
fn load_font(cc: &eframe::CreationContext<'_>) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "Roboto".into(),
        FontData::from_static(include_bytes!("../assets/fonts/Roboto.ttf")).into(),
    );
    fonts.font_data.insert(
        "Roboto-regular".into(),
        FontData::from_static(include_bytes!("../assets/fonts/Roboto-Regular.ttf")).into(),
    );

    let mut newfam = BTreeMap::new();
    newfam.insert(FontFamily::Name("Roboto".into()), vec!["Roboto".to_owned()]);
    newfam.insert(
        FontFamily::Name("Roboto-regular".into()),
        vec!["Roboto-regular".to_owned()],
    );
    fonts.families.append(&mut newfam);

    cc.egui_ctx.set_fonts(fonts);
}
