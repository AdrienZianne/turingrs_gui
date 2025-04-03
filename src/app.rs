use std::{collections::{BTreeMap, HashMap}, path::PathBuf};

use egui::{FontData, FontDefinitions, FontFamily, Rect};
use egui_extras::install_image_loaders;
use poll_promise::Promise;
use rfd::FileHandle;
use turingrs::{turing_machine::{TuringExecutionStep, TuringMachine, TuringMachineExecutor}, turing_state::{TuringDirection, TuringTransition}};

use crate::ui::{self, turing::{State, Transition}};


/// The application data, not refresh after each draw
pub struct TuringApp {
    pub turing: TuringMachineExecutor,
    pub current_step: TuringExecutionStep,
    pub word_input: String,
    pub count: u8,
    pub is_accepted: Option<bool>,
    pub graph_rect: Rect,
    pub states: HashMap<u8, State>,
    pub selection: Selection,
    pub code: String,
    pub promise: Option<Promise<Option<PathBuf>>>,
    pub promise_wasm: Option<Promise<Option<FileHandle>>>
}

#[derive(Default)]
pub struct Selection {
    pub selected_transition: Option<(u8, u8)>,
    pub selected_state: Option<u8>
}

/// Default implementation of TuringApp
impl Default for TuringApp {
    fn default() -> Self {
        
        // initalize the turing machine crate
        let mut turing_machine = TuringMachine::new(1);
        let mut states: HashMap<u8, State> = HashMap::new();
        for (name, index) in turing_machine.name_index_hashmap.iter() {
            states.insert(
                *index,
                State::new(*index, name.to_string())
            );
        }

        let t1 = turing_machine.append_rule_state_by_name("i".to_string(), TuringTransition::create(
            vec!['ç', 'ç'], vec!['ç'], vec![TuringDirection::Right, TuringDirection::Right]
        ), "a".to_string()).unwrap();

        states.get_mut(&0).unwrap().transitions.push(Transition {
            id: t1,
            parent_id: 0,
            text: turing_machine.states[0].transitions[t1 as usize].to_string()
        });

        let (turing_executor, initial_turing_step) = TuringMachineExecutor::new(
            turing_machine, 
            "".to_string()
        ).expect("Error while creating executor");

        // Implement the TuringApp
        Self {
            turing: turing_executor,
            current_step : initial_turing_step,
            word_input: "".to_string(),
            count: 0,
            is_accepted: None,
            graph_rect: Rect::ZERO,
            states: states,
            selection: Selection::default(),
            code: "".to_string(),
            promise: None,
            promise_wasm: None,
        }
    }


}

/// Global setting of egui
impl TuringApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // cc.egui_ctx.set_debug_on_hover(true);

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
    fonts.font_data.insert(
        "RobotoMono-regular".into(),
        FontData::from_static(include_bytes!("../assets/fonts/RobotoMono-Regular.ttf")).into(),
    );

    let mut newfam = BTreeMap::new();
    newfam.insert(
        FontFamily::Name("Roboto".into()),
        vec!["Roboto".to_owned()]
    );
    newfam.insert(
        FontFamily::Name("Roboto-regular".into()),
        vec!["Roboto-regular".to_owned()],
    );
    newfam.insert(
        FontFamily::Name("RobotoMono-regular".into()),
        vec!["RobotoMono-regular".to_owned()],
    );
    fonts.families.append(&mut newfam);

    cc.egui_ctx.set_fonts(fonts);
}
