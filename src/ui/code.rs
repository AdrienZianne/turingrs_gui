use egui::{
    Align, Color32, CornerRadius, Frame, Label, Layout, Margin, Pos2, Response, RichText,
    ScrollArea, Separator, Shape, Stroke, TextEdit, Ui, Visuals, text::LayoutJob, vec2,
};
use egui_flex::{Flex, FlexDirection, item};

use crate::TuringApp;

use super::{component::button, constant::Constant};

pub fn show(app: &mut TuringApp, ui: &mut Ui) {
    // The button to load and compile the app
    
            ui.scope(|ui| {
                ui.ctx().set_visuals(Visuals {
                    ..Default::default()
                });

                Flex::new()
                    .direction(FlexDirection::Horizontal)
                    .wrap(true)
                    .w_full()
                    .show(ui, |flex| {
                        let compile = button(flex.style_mut(), "Compile");
                        let load_file_button = button(flex.style_mut(), "Load file");
                        let load_graph_button = button(flex.style_mut(), "Load Graph");
                        if flex.add(item(), compile).clicked() {
                            // compile
                        }

                        if flex.add(item(), load_graph_button).clicked() {
                            // graph to code algorithm
                        }

                        let res = flex.add(item(), load_file_button);
                        load_file(app, res);
                    });
            });

    ScrollArea::vertical().show(ui, |ui| {
        Frame::new()
        .fill(Constant::BACKGROUND_2)
        .inner_margin(Margin { top: 10, ..Default::default()})
        .show(ui, |ui| {
            ui.horizontal_top(|ui| {
                let x = Frame::new()
                    .inner_margin(Margin {
                        top: 5,
                        left: 5,
                        ..Default::default()
                    })
                    .show(ui, |ui| {
                        let mut lines_number: String = "".to_string();
                        let number_width = Constant::get_width(ui, &Constant::code_font())-5.0;

                        let mut max_width = 0.0;
                        for (i, s) in app.code.lines().enumerate() {
                            let row_per_line = ui.fonts(|f| {
                                let x = f.layout_job(LayoutJob::simple(
                                    s.to_string(),
                                    Constant::code_font(),
                                    Constant::FOREGROUND,
                                    ui.available_width() - max_width - 20.0,
                                ));
                                x.rows.iter().count()
                            });

                            lines_number +=
                                &((i + 1).to_string() + &String::from("\n".repeat(row_per_line)));

                            if (i + 1).to_string().len() as f32 * number_width > max_width {
                                max_width = (i + 1).to_string().len() as f32 * number_width
                            }
                        }

                        lines_number +=
                                &((app.code.lines().count() + 1).to_string() + &String::from("\n"));
                        if (app.code.lines().count() + 1).to_string().len() as f32 * number_width > max_width {
                            max_width = (app.code.lines().count() + 1).to_string().len() as f32 * number_width
                        }

                        let numbers = Label::new(
                            RichText::new(lines_number)
                                .color(Constant::PRIMARY_COLOR)
                                .line_height(Some(Constant::get_heigt(ui, &Constant::code_font())))
                                .font(Constant::small_font()),
                        )
                        .halign(Align::RIGHT)
                        .extend();



                        ui.allocate_ui_with_layout(
                            vec2(max_width, ui.available_height()),
                            Layout::top_down(Align::Max),
                            |ui| {
                                ui.add_sized(
                                    vec2(
                                        max_width,
                                        0.0,
                                    ),
                                    numbers,
                                );
                                ui.allocate_space(vec2(
                                    max_width,
                                    ui.available_height(),
                                ))
                            },
                        );

                    });

                ui.scope(|ui| {
                    ui.visuals_mut().extreme_bg_color = Color32::TRANSPARENT;
                    ui.visuals_mut().selection.stroke = Stroke::NONE;
                    ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::NONE;

                    let code_edit = TextEdit::multiline(&mut app.code)
                        .background_color(Color32::TRANSPARENT)
                        .code_editor()
                        .text_color(Color32::WHITE)
                        .font(Constant::code_font());

                    ui.add_sized(
                        ui.available_size(),
                        code_edit,
                    );
                });
            });
        })
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn load_file(app: &mut TuringApp, res: Response) {
    use std::fs;

    use poll_promise::Promise;
    use rfd::FileDialog;

    if res.clicked() {
        // if let Some(path) = FileDialog::new().add_filter("tm", &["tm"]).pick_file() {
        //     app.code = fs::read_to_string(path).expect("cannot read file");
        // }

        app.promise = Some(Promise::spawn_thread("load_file", || {
            FileDialog::new().add_filter("ext", &["tm"]).pick_file()
        }));
    }

    if let Some(promise) = &app.promise {
        if let Some(path) = (promise).ready() {
            app.code = fs::read_to_string(path.as_ref().unwrap()).expect("cannot read file");
            app.promise = None;
        } else {
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn load_file(app: &mut TuringApp, res: Response) {
    use std::path::PathBuf;

    if res.clicked() {
        app.promise_wasm = Some(Promise::spawn_local(async move {
            AsyncFileDialog::new()
                .add_filter("ext", &["tm"])
                .pick_file()
                .await
        }));
    }

    if let Some(promise) = &app.promise {
        if let Some(path) = (promise).ready() {
            app.code = fs::read_to_string(path.as_ref().unwrap()).expect("cannot read file");
        } else {
        }
    }
}
