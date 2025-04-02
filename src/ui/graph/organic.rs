use std::collections::HashMap;

use egui::Vec2;

use crate::{ui::{constant::Constant, utils}, TuringApp};

pub fn apply_force(app: &mut TuringApp) -> bool {


    let mut forces: HashMap<u8, Vec2> = HashMap::new();

    // register the max force applied on a state to check if the system is stable
    let mut max_force_applied:f32 = 0.0;

    for (i, state_1) in app.states.iter() {
        
        let mut force :f32 = 0.0;
        let mut final_force: Vec2 = Vec2::ZERO;

        for (j, state_2) in app.states.iter() {

            // continue if it's the same state
            if j == i { continue; }

            // true if there is a transition between the two states
            let are_adjacent = app.turing.turing_machine.get_transition_index(*i, *j).is_some()
                || app.turing.turing_machine.get_transition_index(*j, *i).is_some();
                

            let distance = utils::distance(state_1.position, state_2.position);
            let direction = utils::direction(state_1.position, state_2.position);

            // different equations are use based on the adjacency of the states
            if are_adjacent {
                force = utils::attract_force(state_1.position, state_2.position);
            }
            else if distance < Constant::L {
                force = -utils::rep_force(state_1.position, state_2.position);
            };

            // apply the force on the final force vector
            final_force += direction * force;
        }

        // save the highest force applied
        if force > max_force_applied {
            max_force_applied = force;
        }

        // store the compute force to not alter the current physical state
        forces.insert(*i, final_force);
    }

    for (i, state) in app.states.iter_mut() {
        // translate the state by the amount of force
        state.position += *forces.get(&i).unwrap();
    }

    max_force_applied < Constant::STABILITY_TRESHOLD
}