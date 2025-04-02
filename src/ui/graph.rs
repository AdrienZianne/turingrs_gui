use std::collections::{HashMap, hash_map::Entry};

use egui::{Button, Pos2, Rect, Scene, Ui, UiBuilder, Vec2};
use itertools::Itertools;
use organic::apply_force;
use state::draw_states;
use transition::{draw_loop_transitions, draw_normal_transitions};

use crate::TuringApp;

use super::turing::Transition;

mod organic;
mod state;
mod transition;

pub fn show(app: &mut TuringApp, ui: &mut Ui) {
    // current rect of the element inside the scene
    let mut inner_rect = Rect::ZERO;
    
    // current rect displayed of inner_ rect on the scene, use by the library to keep the
    // translation and zoom reference
    let mut scene_rect = app.graph_rect;

    let is_stable = apply_force(app);


    let scene_response = Scene::new()
        .show(ui, &mut scene_rect, |ui| {
            draw_transitions(app, ui);

            draw_states(app, ui);

            inner_rect = ui.min_rect();
        })
        .response;

    // if ui.put(scene_response.rect,Button::image(egui::include_image!("../../assets/center.png"))).clicked() {
    //     scene_rect = inner_rect;
    // }

    // save scene rect information
    app.graph_rect = scene_rect;

    // If the graph canvas is clicked
    if scene_response.clicked() {
        app.selection.selected_state = None;
        app.selection.selected_transition = None;
    }

    // If the graph didn't reach a stable state in the current frame, ask to draw the next even if no user interaction
    if !is_stable || true {
        ui.ctx().request_repaint();
    }
}

/// draw the transitions between states.
/// This function must be called before draw_states to display the states to the top layer.
fn draw_transitions(app: &mut TuringApp, ui: &mut Ui) {
    // group transition by (source, target) index
    let mut transitions_hashmap: HashMap<(u8, u8), Vec<&mut Transition>> = HashMap::new();

    let mut state_position: HashMap<u8, Pos2> = HashMap::new();

    let mut graph_center = Vec2::ZERO;
    let states_count = app.states.len();

    // iterate all states to find transitions
    for (index, state) in app.states.iter_mut() {
        state_position.insert(*index, state.position);

        // each state hold all transitions of which its the source
        for transition in state.transitions.iter_mut() {

            // get TuringTransition to acquire the nex state id
            let target_state_index = app.turing.turing_machine
                .states[*index as usize]
                .transitions[transition.id as usize]
                .index_to_state;

            // if the value exist, add to the vector, if not create a new pair key/value
            match transitions_hashmap.entry((*index, target_state_index)) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(transition);
                }
                Entry::Vacant(e) => {
                    e.insert(vec![transition]);
                }
            }
        }

        // compute graph center based on node position
        graph_center += state.position.to_vec2();
    }

    graph_center /= states_count as f32;


    // iterate each group of transitions
    for ((from, to), transitions) in transitions_hashmap.iter_mut().sorted_by_key(|f| f.0) {
        // vector of transition and a boolean
        let mut transition_rules: Vec<&mut Transition> = vec![];

        for transition in transitions {
            transition_rules.push(transition);
        }

        let source_position = state_position.get(&from).unwrap();
        let target_position = state_position.get(&to).unwrap();

        if from == to {
            // draw loop transition for same state source/target
            draw_loop_transitions(
                &mut app.selection,
                ui,
                *source_position,
                transition_rules,
                graph_center,
            );
        } else {

            let reverse = app.turing.turing_machine.get_transition_index(*to, *from).is_some_and(|_| to > from);
            // draw normal bezier
            draw_normal_transitions(
                &mut app.selection,
                ui,
                *source_position,
                *target_position,
                transition_rules,
                graph_center,
                reverse
            );
        }
    }
}