use crate::colors::{set_theme, Theme, FRAPPE, LATTE, MACCHIATO, MOCHA};
use eframe::egui;
use std::collections::HashMap;
mod colors;
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
    theme: Theme,
    help_opened: bool,
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
    lines: Vec<Vec<[f64; 2]>>,
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
            lines: vec![vec![]],
            plot_max_y: 1.0,
        }
    }
    fn f(&self, x: f64) -> f64 {
        match self.current_function {
            CurrentFunction::First => f64::exp(-x) * f64::cos(x * std::f64::consts::PI),
            CurrentFunction::Second => {
                3.0 * f64::powi(x, 4) - 4.0 * f64::powi(x, 3) - 12.0 * x * x + 2.0
            }
            CurrentFunction::Third => x * x - 5.0 * f64::sin(x),
            CurrentFunction::Fourth => 0.1 * x * x - x * f64::ln(x),
        }
    }
    fn f_der2(&self, x: f64) -> f64 {
        match self.current_function {
            CurrentFunction::First => {
                -(f64::cos(std::f64::consts::PI * x)
                    + std::f64::consts::PI * f64::sin(std::f64::consts::PI))
                    * f64::exp(-x)
            }
            CurrentFunction::Second => 12.0 * f64::powi(x, 3) - 12.0 * f64::powi(x, 2) - 24.0 * x,
            CurrentFunction::Third => 2.0 * x - 5.0 * f64::cos(x),
            CurrentFunction::Fourth => 0.2 * x - f64::ln(x) - 1.0,
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
        use eframe::{
            egui::{FontFamily, TextStyle},
            epaint::FontId,
        };
        use FontFamily::{Monospace, Proportional};
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Heading, FontId::new(19.0, Proportional)),
            (
                TextStyle::Name("Heading2".into()),
                FontId::new(22.0, Proportional),
            ),
            (
                TextStyle::Name("ContextHeading".into()),
                FontId::new(19.0, Proportional),
            ),
            (TextStyle::Body, FontId::new(14.0, Proportional)),
            (TextStyle::Monospace, FontId::new(12.0, Monospace)),
            (TextStyle::Button, FontId::new(16.0, Proportional)),
            (TextStyle::Small, FontId::new(20.0, Proportional)),
        ]
        .into();
        cc.egui_ctx.set_style(style);

        MathApp {
            current_function: CurrentFunction::First,
            functions: vec![
                Function::new(
                    "exp(-x) * cos(πx)",
                    -1.0,
                    1.0,
                    0.001,
                    CurrentFunction::First,
                ),
                Function::new(
                    "3x⁴ - 4x³ - 12x² + 2",
                    -1.0,
                    1.0,
                    0.001,
                    CurrentFunction::Second,
                ),
                Function::new("x² - 5sin(x)", -1.0, 1.0, 0.001, CurrentFunction::Third),
                Function::new("0.1x - xln(x)", -1.0, 1.0, 0.001, CurrentFunction::Fourth),
            ],
            theme: LATTE,
            help_opened: false,
        }
    }
}
impl eframe::App for MathApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        set_theme(ctx, self.theme);
        let current: usize = match self.current_function {
            CurrentFunction::First => 0,
            CurrentFunction::Second => 1,
            CurrentFunction::Third => 2,
            CurrentFunction::Fourth => 3,
        };
        egui::TopBottomPanel::top("Title").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.heading("Chord method Showcase.");
                    ui.label("Source code:");
                    ui.hyperlink_to("🔌GitHub", "https://github.com/bmg-c/opr");
                });
                ui.with_layout(
                    egui::Layout::right_to_left(eframe::emath::Align::Center),
                    |ui| {
                        if ui.add(egui::Button::new("HELP")).clicked() {
                            self.help_opened = if self.help_opened == false {
                                true
                            } else {
                                false
                            };
                        }
                        if ui
                            .add(egui::Button::new(format!(
                                "Theme: {}",
                                match self.theme {
                                    MOCHA => "MOCHA",
                                    MACCHIATO => "MACCHIATO",
                                    FRAPPE => "FRAPPE",
                                    LATTE => "LATTE",
                                    _ => "LATTE",
                                }
                            )))
                            .clicked()
                        {
                            self.theme = match self.theme {
                                MOCHA => MACCHIATO,
                                MACCHIATO => FRAPPE,
                                FRAPPE => LATTE,
                                LATTE => MOCHA,
                                _ => LATTE,
                            }
                        }
                        ui.hyperlink_to("Ivan ", "https://github.com/bmg-c");
                        ui.label("Author:");
                    },
                );
            });
        });
        egui::TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
            ui.label("");
            ui.separator();
            let x_decimals: usize = {
                let mut i: usize = 0;
                let mut dec: usize = 0;
                for part in self.functions[current]
                    .eps
                    .clone()
                    .to_string()
                    .split(".")
                {
                    i += 1;
                    dec = part.len()
                }
                match i {
                    2 => dec,
                    _ => 2,
                }
            } + 1;
            ui.horizontal(|ui| {
                ui.heading(if self.functions[current].current_iteration != 0 {
                    format!(
                        "x = {}, f(x) = {}",
                        format!("{:.1$}", self.functions[current].x2, x_decimals),
                        format!(
                            "{:.1$}",
                            self.functions[current].f(self.functions[current].x2),
                            x_decimals
                        )
                    )
                } else {
                    "".to_string()
                });
                ui.with_layout(
                    egui::Layout::right_to_left(eframe::emath::Align::RIGHT),
                    |ui| {
                        ui.heading(format!(
                            "Iteration {}",
                            self.functions[current].current_iteration
                        ));
                    },
                );
            });
            ui.heading(
                if self.functions[current].is_error
                    || self.functions[current].x2.is_nan()
                    || self.functions[current].x2.is_infinite()
                {
                    "Error happened!"
                } else if self.functions[current].reached_eps {
                    "Reached end!"
                } else {
                    ""
                },
            );
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Choose your equation: ");
                ui.vertical(|ui| {
                    ui.selectable_value(
                        &mut self.current_function,
                        CurrentFunction::First,
                        format!("{} = 0", &self.functions[0].title),
                    );
                    ui.selectable_value(
                        &mut self.current_function,
                        CurrentFunction::Second,
                        format!("{} = 0", &self.functions[1].title),
                    );
                    ui.selectable_value(
                        &mut self.current_function,
                        CurrentFunction::Third,
                        format!("{} = 0", &self.functions[2].title),
                    );
                    ui.selectable_value(
                        &mut self.current_function,
                        CurrentFunction::Fourth,
                        format!("{} = 0", &self.functions[3].title),
                    );
                });
            });
            ui.horizontal(|ui| {
                ui.label("Set data:");
                let a: f64 = self.functions[current].temp_a.clone();
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
                    egui::DragValue::new(&mut self.functions[current].temp_a)
                        .speed(speed)
                        .min_decimals(decimals)
                        .max_decimals(decimals)
                        .update_while_editing(false)
                        .prefix("a: "),
                );
                let a: f64 = self.functions[current].temp_b.clone();
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
                    egui::DragValue::new(&mut self.functions[current].temp_b)
                        .speed(speed)
                        .min_decimals(decimals)
                        .max_decimals(decimals)
                        .update_while_editing(false)
                        .prefix("b: "),
                );
                let a: f64 = self.functions[current].temp_eps.clone();
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
                    egui::DragValue::new(&mut self.functions[current].temp_eps)
                        .speed(speed)
                        .min_decimals(decimals)
                        .max_decimals(decimals)
                        .update_while_editing(false)
                        .clamp_range(0..=i64::MAX)
                        .prefix("eps: "),
                );
                ui.label("Apply data:");
                if ui.add(egui::Button::new("Update")).clicked() {
                    let temp_eps: f64 = self.functions[current].temp_eps;
                    self.functions[current] = Function::new(
                        self.functions[current].title.as_str(),
                        self.functions[current].temp_a,
                        self.functions[current].temp_b,
                        self.functions[current].temp_eps,
                        self.functions[current].current_function,
                    );
                    self.functions[current].eps = temp_eps;
                }
            });
            ui.separator();
            ui.vertical_centered(|ui| {
                ui.label(format!(
                    "a: {}, b: {}, eps: {}",
                    self.functions[current].a,
                    self.functions[current].b,
                    self.functions[current].eps
                ));
            });
            if self.functions[current].current_iteration == -1 {
                let points: usize = 100;
                let left: f64 = self.functions[current].a;
                let right: f64 = self.functions[current].b;
                let mut graph: Vec<[f64; 2]> = vec![[0., 0.]; points];
                let step: f64 = (right - left) / points as f64;
                let mut x: f64 = left;
                graph[0][0] = x;
                graph[0][1] = self.functions[current].f(x);
                x += step;
                let mut plot_max_y: f64 = f64::abs(graph[0][1]);
                for i in 1..points {
                    graph[i][0] = x;
                    graph[i][1] = self.functions[current].f(x);
                    plot_max_y = if plot_max_y < f64::abs(graph[i][1]) {
                        graph[i][1]
                    } else {
                        plot_max_y
                    };
                    x += step;
                }
                self.functions[current]
                    .current_plot_vec
                    .insert(String::from("function"), graph);
                let border_left: Vec<[f64; 2]> = vec![
                    [self.functions[current].a, -plot_max_y],
                    [self.functions[current].a, plot_max_y],
                ];
                self.functions[current]
                    .current_plot_vec
                    .insert(String::from("Left border"), border_left);
                let border_right: Vec<[f64; 2]> = vec![
                    [self.functions[current].b, -plot_max_y],
                    [self.functions[current].b, plot_max_y],
                ];
                self.functions[current]
                    .current_plot_vec
                    .insert(String::from("Right border"), border_right);
                self.functions[current].plot_max_y = plot_max_y;
                self.functions[current].current_iteration += 1;
            }
            egui_plot::Plot::new("My Plot")
                .legend(egui_plot::Legend::default())
                .show(ui, |plot_ui| {
                    let mut is_x1_line: bool = true;
                    for line in self.functions[current].lines.clone().into_iter() {
                        if is_x1_line {
                            plot_ui.line(
                                egui_plot::Line::new(egui_plot::PlotPoints::from(line))
                                    .color(self.theme.green)
                                    .name("Showcase"),
                            );
                        } else {
                            plot_ui.line(
                                egui_plot::Line::new(egui_plot::PlotPoints::from(line))
                                    .color(self.theme.teal)
                                    .name("Showcase"),
                            );
                        };
                        is_x1_line = !is_x1_line;
                    }
                    plot_ui.line(
                        egui_plot::Line::new(egui_plot::PlotPoints::from(
                            match self.functions[current]
                                .current_plot_vec
                                .get("function")
                            {
                                Some(graph) => graph.clone(),
                                None => vec![],
                            },
                        ))
                        .color(self.theme.red)
                        .name(self.functions[current].title.as_str()),
                    );
                    plot_ui.line(
                        egui_plot::Line::new(egui_plot::PlotPoints::from(
                            match self.functions[current]
                                .current_plot_vec
                                .get("Left border")
                            {
                                Some(border) => border.clone(),
                                None => vec![],
                            },
                        ))
                        .color(self.theme.pink)
                        .name("Left and Right borders"),
                    );
                    plot_ui.line(
                        egui_plot::Line::new(egui_plot::PlotPoints::from(
                            match self.functions[current]
                                .current_plot_vec
                                .get("Right border")
                            {
                                Some(border) => border.clone(),
                                None => vec![],
                            },
                        ))
                        .color(self.theme.pink)
                        .name("Left and Right borders"),
                    );
                });
            ui.horizontal_centered(|ui| {
                if ui.add(egui::Button::new("Next iteration")).clicked()
                    && !self.functions[current].reached_eps
                    && !self.functions[current].is_error
                    && !self.functions[current].x2.is_nan()
                    && !self.functions[current].x2.is_infinite()
                {
                    match get_root_chord_method(&mut self.functions[current]) {
                        Some(answer) => {
                            self.functions[current].fixed = answer.fixed;
                            self.functions[current].x2 = answer.x2;
                            self.functions[current].reached_eps = answer.reached_eps;
                            self.functions[current]
                                .lines
                                .append(&mut answer.lines.clone());
                        }
                        None => {
                            self.functions[current].is_error = true;
                        }
                    }
                    self.functions[current].current_iteration += 1;
                }
                if ui.add(egui::Button::new("Solve")).clicked()
                    && !self.functions[current].reached_eps
                    && !self.functions[current].is_error
                    && !self.functions[current].x2.is_nan()
                    && !self.functions[current].x2.is_infinite()
                {
                    loop {
                        match get_root_chord_method(&mut self.functions[current]) {
                            Some(answer) => {
                                self.functions[current].fixed = answer.fixed;
                                self.functions[current].x2 = answer.x2;
                                self.functions[current].current_iteration += 1;
                                self.functions[current]
                                    .lines
                                    .append(&mut answer.lines.clone());
                                if answer.reached_eps {
                                    self.functions[current].reached_eps =
                                        answer.reached_eps;
                                    break;
                                }
                            }
                            None => {
                                self.functions[current].is_error = true;
                                break;
                            }
                        }
                    }
                }
            });
        });
        egui::Window::new("Help") .open(&mut self.help_opened) .show(ctx, |ui| { ui.label("This program solves a nonlinear equation using the chord method."); ui.label("You can choose the equation by selecting it in a \"Choose your equation\" menu and set your data below."); ui.label("Then you can walk through iteration by clicking the \"Next iteration\" button or the \"Solve\" button."); ui.label("Each iteration will show on the graph how it finds each x closer to a real one.") });
    }
}
struct Answer {
    lines: Vec<Vec<[f64; 2]>>,
    reached_eps: bool,
    fixed: f64,
    x2: f64,
}
fn get_root_chord_method(func: &Function) -> Option<Answer> {
    let fixed: f64;
    let x1: f64;
    let x2: f64;
    let mut answer: Answer = Answer {
        lines: vec![],
        reached_eps: false,
        fixed: 0.0,
        x2: 0.0,
    };
    if func.current_iteration == 0 {
        if func.f(func.a) * func.f_der2(func.a) > 0.0 {
            fixed = func.a;
            // x1 = func.b;
            // x2 = x1 - (func.f(x1) / (func.f(x1) - func.f(fixed))) * (x1 - fixed);
            x2 = func.b;
        } else {
            fixed = func.b;
            // x1 = func.a;
            // x2 = x1 - (func.f(x1) / (func.f(fixed) - func.f(x1))) * (fixed - x1);
            x2 = func.a;
        }
    } else {
        if func.f(func.a) * func.f_der2(func.a) > 0.0 {
            fixed = func.fixed;
            x1 = func.x2;
            x2 = x1 - (func.f(x1) / (func.f(x1) - func.f(fixed))) * (x1 - fixed);
        } else {
            fixed = func.fixed;
            x1 = func.x2;
            x2 = x1 - (func.f(x1) / (func.f(fixed) - func.f(x1))) * (fixed - x1);
        }
        if (x2 - x1).abs() <= func.eps {
            answer.reached_eps = true;
        }
    }
    if x2 > func.b || x2 < func.a {
        return None;
    }
    answer.fixed = fixed;
    answer.x2 = x2;
    answer.lines = vec![
        vec![[fixed, func.f(fixed)], [x2, func.f(x2)]],
        vec![[x2, func.f(x2)], [x2, 0.0]],
    ];
    Some(answer)
}
