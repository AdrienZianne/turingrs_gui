
use egui::{vec2, Align, Color32, Key, Label, Rect, Response, RichText, Sense, Stroke, TextEdit, Ui};
use turingrs::turing_state::{TuringDirection, TuringTransition};

use crate::{app::Selection, ui::{constant::Constant, turing::{State, Transition}, utils::constrast_color}, TuringApp};


pub fn draw_states(app: &mut TuringApp, ui: &mut Ui) {
    
    // use of refcell means i can borrow one of the element of states without 
    // using a second loop
    let keys: Vec<u8> = app.states.keys().map(|u| *u).collect::<Vec<u8>>();
    for i in keys {

        let state = app.states.get_mut(&i).unwrap();

        let response = draw_node(
            &mut app.selection,
            ui,
            state,
            false
        );

        // if the current state is clicked
        if response.clicked() {

            // and a state is already selected, then create a transition between them
            // from selected to current state.
            if let Some(selected) = app.selection.selected_state {

                let transition = TuringTransition::new(
                    vec!['ç'; app.turing.turing_machine.k as usize + 1], 
                    TuringDirection::Right, 
                    vec![('ç', TuringDirection::Right); app.turing.turing_machine.k as usize]
                );

                // get the string representation of the transition rule
                let transition_string = transition.to_string();

                // get the id of the transition after adding it
                let transition_id = app.turing.turing_machine.append_rule_state(selected, transition, i)
                    .expect("Unable to add rule");

                // if the selected state is the same as the current one, no need to get_mut()
                let state = if selected == i {state} else {app.states.get_mut(&selected).unwrap()};

                // add the transition to the graphical
                state.transitions.push(
                    Transition {
                        text: transition_string,
                        id: transition_id,
                        parent_id: selected,
                    }
                );

                app.selection.selected_state = None;
                
            } 
            // else select the current state 
            else {
                app.selection.selected_state = Some(i);
                app.selection.selected_transition = None;
            }
        }

        // make the state follow the cursor when dragged
        if response.dragged() {
            app.states.get_mut(&i).unwrap().position = response.interact_pointer_pos().unwrap();
        }

        // remove the selection if the key enter is pressed.
        // if the name is already taken add a number
        if response.ctx.input(|input| {
            input.key_pressed(Key::Enter)
        }) {
            if let Some(id) = app.selection.selected_state {
                let mut state_name = app.states.get(&id).unwrap().name.to_string();

                while app.states.iter().any(|(o_id, o_state)| { *o_id != id && o_state.name == state_name }) {
                    state_name += "2";
                }
                app.states.get_mut(&id).unwrap().name = state_name;


            }
            app.selection.selected_state = None
        }
    }
}

/// Draw a single state
fn draw_node(
    selection: &mut Selection,
    ui: &mut Ui,
    state: &mut State,
    is_current: bool
) -> Response {
    
    let rect = Rect::from_center_size(
        state.position, 
        vec2(Constant::STATE_RADIUS, Constant::STATE_RADIUS) * 2.0
    );

    let color = if is_current {Constant::SELECTED} else {state.color};

    ui.painter().circle(
        state.position, 
        Constant::STATE_RADIUS,
        color,
        Stroke::new(3.0,constrast_color(color))
    );

    // if there is a state selected and it's the same as this one
    if selection.selected_state.is_some_and(|selected_state| selected_state == state.id) {

        let response = ui.scope(|ui| {
            
            ui.visuals_mut().extreme_bg_color = Color32::TRANSPARENT;
            ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(1.0, constrast_color(Constant::PRIMARY_COLOR));
            ui.visuals_mut().selection.stroke = Stroke::NONE;

            ui.put(
                rect,
                TextEdit::singleline(&mut state.name)
                    .font(Constant::big_font())
                    .horizontal_align(Align::Center)
                    .vertical_align(Align::Center),
            )
        }).inner;

        response.request_focus();

        response

    } else {

        let mut text = RichText::new(&state.name)
        .font(Constant::big_font())
        .color(constrast_color(color));

        if is_current {text = text.underline()}
        let label = Label::new(text).truncate();

        ui.put(rect, label);

        ui.allocate_rect(rect, Sense::click_and_drag())
    }
}