use egui::{
    epaint::{CubicBezierShape, PathShape, QuadraticBezierShape}, vec2, Align, Color32, CornerRadius, Label, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, TextEdit, Ui, Vec2
};

use crate::{
    app::Selection,
    ui::{constant::Constant, turing::Transition, utils},
};


/// Draw transitions between 2 different states with a quadractic bezier.
pub fn draw_normal_transitions(
    selection: &mut Selection,
    ui: &mut Ui,
    source: Pos2,
    target: Pos2,
    transitions: Vec<&mut Transition>,
    graph_center: Vec2,
    reverse: bool
) {
    // the perpendicular vector to the vector between the position of the 2 states
    let mut delta = (source - target).rot90().normalized();

    // the center of the vector between the position of the 2 states
    let center = vec2((source.x + target.x) / 2.0, (source.y + target.y) / 2.0);

    // flip the transition
    if utils::distance((center + delta).to_pos2(), graph_center.to_pos2()) < utils::distance((center - delta).to_pos2(), graph_center.to_pos2()) 
    {
        delta = -delta
    }

    // force flip the transition back
    if reverse { delta = -delta}

    // the 3 controls points of the quadratic bezier
    let points = [
        source,
        (center + delta * Constant::TRANSITION_CURVATURE * 2.0).to_pos2(),
        target
    ];

    // draw the bezier
    ui.painter().add(QuadraticBezierShape::from_points_stroke(
        points,
        false,
        Color32::TRANSPARENT,
        Stroke::new(Constant::TRANSITION_THICKNESS, Constant::PRIMARY_COLOR),
    ));

    let curve_lenght = get_quadratic_len(points, 100);
    let arrow_position = quadraticbeziercurve(points, map(&curve_lenght, 100, 1.0 - Constant::STATE_RADIUS/curve_lenght.last().unwrap()));
    let arrow_direction = (arrow_position - target.to_vec2()).normalized();

    // points of the triangle
    let triangles = vec![
        arrow_position.to_pos2(),
        (arrow_position
            + arrow_direction * Constant::ARROW_SIZE
            + arrow_direction.rot90() * Constant::ARROW_SIZE / 2.0)
            .to_pos2(),
        (arrow_position + arrow_direction * Constant::ARROW_SIZE
            - arrow_direction.rot90() * Constant::ARROW_SIZE / 2.0)
            .to_pos2(),
    ];

    // draw the triangle
    ui.painter().add(PathShape::convex_polygon(
        triangles,
        Constant::PRIMARY_COLOR,
        Stroke::NONE,
    ));
    
    let rules_len = transitions[0].text.len();
    let offset = vec2(
        Constant::TRANSITION_CURVATURE + rules_len as f32 * Constant::get_width(ui, &Constant::default_font()) / 2.0, 
        Constant::TRANSITION_CURVATURE + transitions.len() as f32 * (Constant::get_heigt(ui, &Constant::default_font()) - 10.0)
    );

    draw_labels(
        selection,
        ui,
        center.to_pos2(),
        transitions,
        (center + delta * offset).to_pos2(),
    );
}


/// Draw transitions between the same state with a cubic bezier.
pub fn draw_loop_transitions(
    selection: &mut Selection,
    ui: &mut Ui,
    source: Pos2,
    transitions: Vec<&mut Transition>,
    graph_center: Vec2,
) {
    // delta based on the center and the only state position
    let delta = (source.to_vec2() - graph_center).normalized();

    let size = 150.0;

    // the four control points of the cubic bezier
    let points = [
        source,
        source + delta * size + delta.rot90() * size/2.0,
        source + delta * size - delta.rot90() * size/2.0,
        source,
    ];

    // draw the bezier
    ui.painter().add(CubicBezierShape::from_points_stroke(
        points,
        false,
        Color32::TRANSPARENT,
        Stroke::new(Constant::TRANSITION_THICKNESS, Constant::PRIMARY_COLOR),
    ));

    // we get the arrow position on the curve
    let curve_lenght = get_cubic_len(points, 100);
    let arrow_position = cubicbeziercurve(points, map(&curve_lenght, 100, 1.0 - Constant::STATE_RADIUS/curve_lenght.last().unwrap()));
    let arrow_direction = (arrow_position - source.to_vec2()).normalized();

    // points of the triangle
    let triangles = vec![
        arrow_position.to_pos2(),
        (arrow_position
            + arrow_direction * Constant::ARROW_SIZE
            + arrow_direction.rot90() * Constant::ARROW_SIZE / 2.0)
            .to_pos2(),
        (arrow_position + arrow_direction * Constant::ARROW_SIZE
            - arrow_direction.rot90() * Constant::ARROW_SIZE / 2.0)
            .to_pos2(),
    ];

    // draw the triangle
    ui.painter().add(PathShape::convex_polygon(
        triangles,
        Constant::PRIMARY_COLOR,
        Stroke::NONE,
    ));

    let text_position = vec2(source.x + delta.x * size/2.0, source.y + delta.y * size/2.0).to_pos2();

    draw_labels(selection, ui, source, transitions, text_position);
}



/// draw the transitions rules as superposed label
fn draw_labels(
    mut selection: &mut Selection,
    ui: &mut Ui,
    source: Pos2,
    mut transitions: Vec<&mut Transition>,
    position: Pos2,
) {

    let font_height = Constant::get_heigt(ui, &Constant::default_font());
    let height_used = transitions.len() as f32 * font_height;
    // enumerate the transition
    let mut i: usize = 0;

    ui.painter().circle(position, 2.0, Color32::CYAN, Stroke::NONE);
    for transition in transitions {

        // initialise the rectangle where the text will be
        let max_rect = Rect::from_center_size(
            position + vec2(0.0, -(height_used/2.0 - (i as f32 + 0.5) * font_height)),
            vec2((transition.text.len() + 5) as f32 * Constant::get_width(ui, &Constant::default_font()), Constant::get_heigt(ui, &Constant::default_font())),
        );

        // draw the text or a single line text edit if selected, then return the rect used
        let rect = if selection.selected_transition.is_some_and(|selected_transition| selected_transition == (transition.parent_id, transition.id)) {
            
            // scope to not affect style of other parts
            let response = ui.scope(|ui| {
                ui.visuals_mut().extreme_bg_color = Color32::TRANSPARENT;
                ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(1.0, utils::constrast_color(Constant::PRIMARY_COLOR));
                ui.visuals_mut().selection.stroke = Stroke::NONE;

                ui.put(max_rect, TextEdit::singleline(&mut transition.text)
                    .font(Constant::default_font())
                    .text_color(utils::constrast_color(Constant::BACKGROUND_2))
                    .horizontal_align(Align::Center)
                    .vertical_align(Align::Center)
                )
            }).inner;

            response.request_focus();

            response

        } else {
            let mut text = RichText::new(&transition.text)
                .font(Constant::default_font())
                .color(Constant::PRIMARY_COLOR);
            
            if selection
                .selected_transition
                .is_some_and(|selected_transition| {
                    selected_transition == (transition.parent_id, transition.id)
                }) {
                text = text.color(Constant::SELECTED);
            }

            // draw the text
            ui.put(max_rect, Label::new(text).extend())
        }.rect;

        // add a click listener to the rectangle of the label/textedit
        let response = ui.allocate_rect(rect, Sense::click());

        // if a transition rule is clicked, then we set it as selected
        if response.clicked() {
            selection.selected_transition = Some((transition.parent_id, transition.id));
            selection.selected_state = None;
        }

        i += 1;
    }
}



/// return a point on the curve of a quadratic bezier
fn quadraticbeziercurve(points: [Pos2; 3], t: f32) -> Vec2 {
    let x = (1.0 - t).powi(2) * points[0].x
        + 2.0 * (1.0 - t) * t * points[1].x
        + t.powi(2) * points[2].x;
    let y = (1.0 - t).powi(2) * points[0].y
        + 2.0 * (1.0 - t) * t * points[1].y
        + t.powi(2) * points[2].y;
    Vec2::new(x, y)
}



/// return a point on the curve of a cubic bezier
fn cubicbeziercurve(points: [Pos2; 4], t: f32) -> Vec2 {
    let x = (1.0 - t).powi(3) * points[0].x
        + 3.0 * (1.0 - t).powi(2) * t * points[1].x
        + 3.0 * (1.0 - t) * t.powi(2) * points[2].x
        + t.powi(3) * points[3].x;
    let y = (1.0 - t).powi(3) * points[0].y
        + 3.0 * (1.0 - t).powi(2) * t * points[1].y
        + 3.0 * (1.0 - t) * t.powi(2) * points[2].y
        + t.powi(3) * points[3].y;
    Vec2::new(x, y)
}

fn get_cubic_len(points: [Pos2; 4], n: usize) -> Vec<f32> {
    let mut arc_length: Vec<f32> = vec![0.0; n+1];

    let mut origin = cubicbeziercurve(points, 0.0);
    let mut clen = 0.0;
    for i in 1..n+1 {
        let pos = cubicbeziercurve(points, i as f32 * (1.0/n as f32));
        let delta = origin - pos;
        clen += (delta.x.powi(2) + delta.y.powi(2)).sqrt();
        arc_length[i as usize] = clen;
        origin = pos;
    }

    arc_length
}

fn get_quadratic_len(points: [Pos2; 3], n: usize) -> Vec<f32> {
    let mut arc_length: Vec<f32> = vec![0.0; n+1];

    let mut origin = quadraticbeziercurve(points, 0.0);
    let mut clen = 0.0;
    for i in 1..n+1 {
        let pos = quadraticbeziercurve(points, i as f32 / (n+1) as f32);
        let delta = origin - pos;
        clen += (delta.x.powi(2) + delta.y.powi(2)).sqrt();
        arc_length[i as usize] = clen;
        origin = pos;
    }

    arc_length
}


fn map(len: &Vec<f32>, n: usize, t: f32) -> f32 {
    let target = t * len[n-1];
    let mut low = 0;
    let mut high = n;
    let mut i = 0;

    while low < high {
        i = low + ((high - low) / 2 | 0);

        if len[i] < target {
            low = i + 1;
        } else {
            high = i;
        }
    }

    if len[i] > target {
        i -= 1;
    }

    let before = len[i];

    if before == target {
        i as f32 / n as f32
    } else {
        (i as f32 + (target - before) as f32 / (len[i + 1] - before)) / n as f32
    }
}
