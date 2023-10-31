use eframe::{
    egui::{self, FontFamily, TextStyle},
    epaint::FontId,
};
use std::collections::HashMap;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "MathApp",
        native_options,
        Box::new(|cc| Box::new(MathApp::new(cc))),
    )
}

struct MathApp {
    current_function: CurrentFunction,
    functions: Vec<Function>,
}

struct Function {
    title: String,
    temp_a: f64,
    temp_b: f64,
    temp_eps: f64,
    a: f64,
    b: f64,
    eps: f64,
    current_function: CurrentFunction,
    fixed: f64,
    x2: f64,
    current_iteration: i64,
    reached_eps: bool,
    is_error: bool,
    current_plot_vec: HashMap<String, Vec<[f64; 2]>>,
    plot_max_y: f64,
}
impl Function {
    fn new(name: &str, left: f64, right: f64, precision: f64, place: CurrentFunction) -> Function {
        Function {
            title: name.to_string(),
            temp_a: left,
            temp_b: right,
            temp_eps: precision,
            a: left,
            b: right,
            eps: precision,
            current_function: place,
            fixed: right,
            x2: right,
            current_iteration: -1,
            reached_eps: false,
            is_error: false,
            current_plot_vec: HashMap::<String, Vec<[f64; 2]>>::new(),
            plot_max_y: 1.0,
        }
    }
    fn f(&self, x: f64) -> f64 {
        match self.current_function {
            CurrentFunction::First => f64::exp(-x) * f64::cos(x * std::f64::consts::PI),
            CurrentFunction::Second => x * x * x,
            CurrentFunction::Third => x,
            CurrentFunction::Fourth => x * x * x * x * x,
        }
    }
    fn f_der2(&self, x: f64) -> f64 {
        match self.current_function {
            CurrentFunction::First => {
                -(f64::cos(std::f64::consts::PI * x)
                    + std::f64::consts::PI * f64::sin(std::f64::consts::PI))
                    * f64::exp(-x)
            }
            CurrentFunction::Second => 3.0 * x * x,
            CurrentFunction::Third => 1.0,
            CurrentFunction::Fourth => 5.0 * x * x * x * x,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum CurrentFunction {
    First,
    Second,
    Third,
    Fourth,
}

impl MathApp {
    fn new(cc: &eframe::CreationContext<'_>) -> MathApp {
        // use FontFamily::{Monospace, Proportional};
        // let mut style = (*cc.egui_ctx.style()).clone();
        // style.text_styles = [
        //     (TextStyle::Heading, FontId::new(25.0, Proportional)),
        //     (TextStyle::Name("Heading2".into()), FontId::new(22.0, Proportional)),
        //     (TextStyle::Name("ContextHeading".into()), FontId::new(19.0, Proportional)),
        //     (TextStyle::Body, FontId::new(16.0, Proportional)),
        //     (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        //     (TextStyle::Button, FontId::new(16.0, Proportional)),
        //     (TextStyle::Small, FontId::new(8.0, Proportional)),
        // ]
        // .into();
        // cc.egui_ctx.set_style(style);
        MathApp {
            current_function: CurrentFunction::First,
            functions: vec![
                Function::new("x^2", -1.0, 1.0, 0.001, CurrentFunction::First),
                Function::new("x^3", -1.0, 1.0, 0.001, CurrentFunction::Second),
                Function::new("x^4", -1.0, 1.0, 0.001, CurrentFunction::Third),
                Function::new("x^5", -1.0, 1.0, 0.001, CurrentFunction::Fourth),
            ],
        }
    }
}

impl eframe::App for MathApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let current_function: usize = match self.current_function {
            CurrentFunction::First => 0,
            CurrentFunction::Second => 1,
            CurrentFunction::Third => 2,
            CurrentFunction::Fourth => 3,
        };
        egui::TopBottomPanel::top("Function select").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.selectable_value(
                    &mut self.current_function,
                    CurrentFunction::First,
                    &self.functions[0].title,
                );
                ui.selectable_value(
                    &mut self.current_function,
                    CurrentFunction::Second,
                    &self.functions[1].title,
                );
                ui.selectable_value(
                    &mut self.current_function,
                    CurrentFunction::Third,
                    &self.functions[2].title,
                );
                ui.selectable_value(
                    &mut self.current_function,
                    CurrentFunction::Fourth,
                    &self.functions[3].title,
                );
            });
        });
        egui::TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
            ui.label("");
            ui.separator();
            ui.heading(format!(
                "Iteration: {}, x = {}",
                self.functions[current_function].current_iteration,
                self.functions[current_function].x2
            ));
            ui.heading(
                if self.functions[current_function].is_error
                    || self.functions[current_function].x2.is_nan()
                    || self.functions[current_function].x2.is_infinite()
                {
                    "Error happened!"
                } else if self.functions[current_function].reached_eps {
                    "Reached end!"
                } else {
                    ""
                },
            );
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("{} = 0", &self.functions[current_function].title));
            ui.horizontal(|ui| {
                let a: f64 = self.functions[current_function].temp_a.clone();
                let decimals: usize = {
                    let mut i: usize = 0;
                    let mut dec: usize = 0;
                    for part in a.to_string().split(".") {
                        i += 1;
                        dec = part.len()
                    }
                    match i {
                        2 => dec,
                        _ => 2,
                    }
                };
                let speed: f64 = 0.01;
                ui.add(
                    egui::DragValue::new(&mut self.functions[current_function].temp_a)
                        .speed(speed)
                        .min_decimals(decimals)
                        .max_decimals(decimals)
                        .update_while_editing(false)
                        .prefix("a: "),
                );
                let a: f64 = self.functions[current_function].temp_b.clone();
                let decimals: usize = {
                    let mut i: usize = 0;
                    let mut dec: usize = 0;
                    for part in a.to_string().split(".") {
                        i += 1;
                        dec = part.len()
                    }
                    match i {
                        2 => dec,
                        _ => 2,
                    }
                };
                let speed: f64 = 0.01;
                ui.add(
                    egui::DragValue::new(&mut self.functions[current_function].temp_b)
                        .speed(speed)
                        .min_decimals(decimals)
                        .max_decimals(decimals)
                        .update_while_editing(false)
                        .prefix("b: "),
                );
                let a: f64 = self.functions[current_function].temp_eps.clone();
                let decimals: usize = {
                    let mut i: usize = 0;
                    let mut dec: usize = 0;
                    for part in a.to_string().split(".") {
                        i += 1;
                        dec = part.len()
                    }
                    match i {
                        2 => dec,
                        _ => 2,
                    }
                };
                let speed: f64 = 0.01;
                ui.add(
                    egui::DragValue::new(&mut self.functions[current_function].temp_eps)
                        .speed(speed)
                        .min_decimals(decimals)
                        .max_decimals(decimals)
                        .update_while_editing(false)
                        .prefix("eps: "),
                );
                if ui.add(egui::Button::new("Update")).clicked() {
                    let temp_eps: f64 = self.functions[current_function].temp_eps;
                    self.functions[current_function] = Function::new(
                        self.functions[current_function].title.as_str(),
                        self.functions[current_function].temp_a,
                        self.functions[current_function].temp_b,
                        self.functions[current_function].temp_eps,
                        self.functions[current_function].current_function,
                    );
                    self.functions[current_function].eps = temp_eps;
                }
            });
            ui.separator();
            // ui.horizontal(|ui| {
            ui.label(format!(
                "a: {}, b: {}, eps: {}",
                self.functions[current_function].a,
                self.functions[current_function].b,
                self.functions[current_function].eps
            ));
            // });
            if self.functions[current_function].current_iteration == -1 {
                let points: usize = 100;
                let left: f64 = self.functions[current_function].a;
                let right: f64 = self.functions[current_function].b;
                let mut graph: Vec<[f64; 2]> = vec![[0., 0.]; points];

                let step: f64 = (right - left) / points as f64;
                let mut x: f64 = left;
                graph[0][0] = x;
                graph[0][1] = self.functions[current_function].f(x);
                x += step;
                let mut plot_max_y: f64 = f64::abs(graph[0][1]);
                for i in 1..points {
                    graph[i][0] = x;
                    graph[i][1] = self.functions[current_function].f(x);
                    plot_max_y = if plot_max_y < f64::abs(graph[i][1]) {
                        graph[i][1]
                    } else {
                        plot_max_y
                    };
                    x += step;
                }
                self.functions[current_function]
                    .current_plot_vec
                    .insert(String::from("function"), graph);

                let border_left: Vec<[f64; 2]> = vec![
                    [self.functions[current_function].a, -plot_max_y],
                    [self.functions[current_function].a, plot_max_y],
                ];
                self.functions[current_function]
                    .current_plot_vec
                    .insert(String::from("Left border"), border_left);
                let border_right: Vec<[f64; 2]> = vec![
                    [self.functions[current_function].b, -plot_max_y],
                    [self.functions[current_function].b, plot_max_y],
                ];
                self.functions[current_function]
                    .current_plot_vec
                    .insert(String::from("Right border"), border_right);

                self.functions[current_function].plot_max_y = plot_max_y;
                self.functions[current_function].current_iteration += 1;
            }

            egui_plot::Plot::new("My Plot")
                .legend(egui_plot::Legend::default())
                .show(ui, |plot_ui| {
                    let mut x_vec: [f64; 2] = [0.0, 0.0];
                    if match self.functions[current_function]
                        .current_plot_vec
                        .get("lines_x2")
                    {
                        Some(lines) => {
                            if lines.len() != 0 {
                                x_vec = lines[1];
                            }
                            true
                        }
                        None => {
                            false
                        }
                    } {
                        plot_ui.text(
                            egui_plot::Text::new(
                                egui_plot::PlotPoint::from(x_vec),
                                format!("{}", self.functions[current_function].x2),
                            )
                            .name("answer"),
                        );
                    }
                    plot_ui.line(
                        egui_plot::Line::new(egui_plot::PlotPoints::from(
                            match self.functions[current_function]
                                .current_plot_vec
                                .get("function")
                            {
                                Some(graph) => graph.clone(),
                                None => vec![],
                            },
                        ))
                        .name("function"),
                    );
                    plot_ui.line(
                        egui_plot::Line::new(egui_plot::PlotPoints::from(
                            match self.functions[current_function]
                                .current_plot_vec
                                .get("lines_x1")
                            {
                                Some(lines) => lines.clone(),
                                None => vec![],
                            },
                        ))
                        .name("lines"),
                    );
                    plot_ui.line(
                        egui_plot::Line::new(egui_plot::PlotPoints::from(
                            match self.functions[current_function]
                                .current_plot_vec
                                .get("lines_x2")
                            {
                                Some(lines) => lines.clone(),
                                None => vec![],
                            },
                        ))
                        .name("lines"),
                    );
                    plot_ui.line(
                        egui_plot::Line::new(egui_plot::PlotPoints::from(
                            match self.functions[current_function]
                                .current_plot_vec
                                .get("Left border")
                            {
                                Some(border) => border.clone(),
                                None => vec![],
                            },
                        ))
                        .name("function"),
                    );
                    plot_ui.line(
                        egui_plot::Line::new(egui_plot::PlotPoints::from(
                            match self.functions[current_function]
                                .current_plot_vec
                                .get("Right border")
                            {
                                Some(border) => border.clone(),
                                None => vec![],
                            },
                        ))
                        .name("function"),
                    );
                });
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Next iteration")).clicked()
                    && !self.functions[current_function].reached_eps
                    && !self.functions[current_function].is_error
                    && !self.functions[current_function].x2.is_nan()
                    && !self.functions[current_function].x2.is_infinite()
                {
                    match get_root_chord_method(&mut self.functions[current_function]) {
                        Some(answer) => {
                            self.functions[current_function].fixed = answer.fixed;
                            self.functions[current_function].x2 = answer.x2;
                            self.functions[current_function].reached_eps = answer.reached_eps;
                            self.functions[current_function]
                                .current_plot_vec
                                .insert(String::from("lines_x1"), answer.lines[0].clone());
                            self.functions[current_function]
                                .current_plot_vec
                                .insert(String::from("lines_x2"), answer.lines[1].clone());
                        }
                        None => {
                            self.functions[current_function].is_error = true;
                            self.functions[current_function]
                                .current_plot_vec
                                .insert(String::from("lines_x1"), vec![]);
                            self.functions[current_function]
                                .current_plot_vec
                                .insert(String::from("lines_x2"), vec![]);
                        }
                    }
                    self.functions[current_function].current_iteration += 1;
                }
                if ui.add(egui::Button::new("Solve")).clicked()
                    && !self.functions[current_function].reached_eps
                    && !self.functions[current_function].is_error
                    && !self.functions[current_function].x2.is_nan()
                    && !self.functions[current_function].x2.is_infinite()
                {
                    loop {
                        match get_root_chord_method(&mut self.functions[current_function]) {
                            Some(answer) => {
                                self.functions[current_function].fixed = answer.fixed;
                                self.functions[current_function].x2 = answer.x2;
                                self.functions[current_function].current_iteration += 1;
                                if answer.reached_eps {
                                    self.functions[current_function].reached_eps =
                                        answer.reached_eps;
                                    self.functions[current_function]
                                        .current_plot_vec
                                        .insert(String::from("lines_x1"), answer.lines[0].clone());
                                    self.functions[current_function]
                                        .current_plot_vec
                                        .insert(String::from("lines_x2"), answer.lines[1].clone());
                                    break;
                                }
                            }
                            None => {
                                self.functions[current_function].is_error = true;
                                self.functions[current_function]
                                    .current_plot_vec
                                    .insert(String::from("lines_x1"), vec![]);
                                self.functions[current_function]
                                    .current_plot_vec
                                    .insert(String::from("lines_x2"), vec![]);
                                break;
                            }
                        }
                    }
                }
            });
        });
    }
}

struct Answer {
    lines: Vec<Vec<[f64; 2]>>,
    reached_eps: bool,
    fixed: f64,
    x2: f64,
}

fn get_root_chord_method(function: &Function) -> Option<Answer> {
    let fixed: f64;
    let x1: f64;
    let x2: f64;

    let mut answer: Answer = Answer {
        lines: vec![],
        reached_eps: false,
        fixed: 0.0,
        x2: 0.0,
    };

    if function.current_iteration == 0 {
        if function.f(function.a) * function.f_der2(function.a) > 0.0 {
            fixed = function.a;
            x1 = function.b;
            x2 = x1 - (function.f(x1) / (function.f(x1) - function.f(fixed))) * (x1 - fixed);
        } else {
            fixed = function.b;
            x1 = function.a;
            x2 = x1 - (function.f(x1) / (function.f(fixed) - function.f(x1))) * (fixed - x1);
        }
    } else {
        if function.f(function.a) * function.f_der2(function.a) > 0.0 {
            fixed = function.fixed;
            x1 = function.x2;
            x2 = x1 - (function.f(x1) / (function.f(x1) - function.f(fixed))) * (x1 - fixed);
        } else {
            fixed = function.fixed;
            x1 = function.x2;
            x2 = x1 - (function.f(x1) / (function.f(fixed) - function.f(x1))) * (fixed - x1);
        }
        if (x2 - x1).abs() <= function.eps {
            answer.reached_eps = true;
        }
    }
    if x2 > function.b || x2 < function.a {
        return None;
    }
    answer.fixed = fixed;
    answer.x2 = x2;
    answer.lines = vec![
        vec![
            [fixed, function.f(fixed)],
            if fixed == function.b {
                [
                    function.a,
                    ((function.f(x1) - function.f(fixed)) / (x1 - fixed)) * (function.a - x1)
                        + function.f(x1),
                ]
            } else {
                [
                    function.b,
                    ((function.f(x1) - function.f(fixed)) / (x1 - fixed)) * (function.a - x1)
                        + function.f(x1),
                ]
            },
        ],
        vec![[x2, function.f(x2)], [x2, 0.0]],
    ];

    Some(answer)
}
