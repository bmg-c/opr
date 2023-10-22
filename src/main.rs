use eframe::egui;
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
    current_iteration: i64,
    current_plot_vec: HashMap<String, Vec<[f64; 2]>>,
    plot_max_y: f64,
}
impl Function {
    fn new(name: &str, left: f64, right: f64, place: CurrentFunction) -> Function {
        Function {
            title: name.to_string(),
            temp_a: left,
            temp_b: left,
            temp_eps: 0.0001,
            a: left,
            b: right,
            eps: 0.0001,
            current_function: place,
            current_iteration: 0,
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

#[derive(PartialEq, Eq)]
enum CurrentFunction {
    First,
    Second,
    Third,
    Fourth,
}

impl MathApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> MathApp {
        MathApp {
            current_function: CurrentFunction::First,
            functions: vec![
                Function::new("x^2", -1.0, 1.0, CurrentFunction::First),
                Function::new("x^3", -1.0, 1.0, CurrentFunction::Second),
                Function::new("x^4", -1.0, 1.0, CurrentFunction::Third),
                Function::new("x^5", -1.0, 1.0, CurrentFunction::Fourth),
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
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.functions[current_function].title);
            ui.add(
                egui::DragValue::new(&mut self.functions[current_function].temp_a)
                    .speed(0.01)
                    .min_decimals(2)
                    .prefix("a: "),
            );
            ui.add(
                egui::DragValue::new(&mut self.functions[current_function].temp_b)
                    .speed(0.01)
                    .min_decimals(2)
                    .prefix("b: "),
            );
            ui.add(
                egui::DragValue::new(&mut self.functions[current_function].temp_eps)
                    .speed(0.01)
                    .min_decimals(5)
                    .prefix("eps: "),
            );
            if ui.add(egui::Button::new("Update")).clicked() {
                self.functions[current_function].a = self.functions[current_function].temp_a;
                self.functions[current_function].b = self.functions[current_function].temp_b;
                self.functions[current_function].eps = self.functions[current_function].temp_eps;
                self.functions[current_function].current_iteration = 0;
            }
            ui.label(format!("Iteration: {}", self.functions[current_function].current_iteration));
            if ui.add(egui::Button::new("Next iteration")).clicked() {
                self.functions[current_function].current_iteration += 1;
            }
            println!(
                "a: {}, b: {}, eps: {}",
                self.functions[current_function].a,
                self.functions[current_function].b,
                self.functions[current_function].eps
            );
        });
        egui::Window::new("Function plot").show(ctx, |ui| {
            if self.functions[current_function].current_iteration == 0 {
                let points: usize = 100;
                let left: f64 = -2.0;
                let right: f64 = 2.0;
                let mut graph: Vec<[f64; 2]> = vec![[0., 0.]; points];

                let step: f64 = (right - left) / points as f64;
                let mut x: f64 = left;
                graph[0][0] = x;
                graph[0][1] = self.functions[current_function].f(x);
                let mut plot_y_max: f64 = f64::abs(graph[0][1]);
                for i in 1..points {
                    graph[i][0] = x;
                    graph[i][1] = self.functions[current_function].f(x);
                    plot_y_max = if plot_y_max < f64::abs(graph[i][1]) {
                        graph[i][1]
                    } else {
                        plot_y_max
                    };
                    x += step;
                }
                self.functions[current_function]
                    .current_plot_vec
                    .insert(String::from("function"), graph);
                self.functions[current_function].plot_max_y = plot_y_max;
                self.functions[current_function].current_iteration += 1;
            }

            // let lines: Vec<[f64; 2]> = vec![
            //     [
            //         get_root_chord_method(&self.functions[current_function]),
            //         -self.functions[current_function].plot_max_y,
            //     ],
            //     [
            //         get_root_chord_method(&self.functions[current_function]),
            //         self.functions[current_function].plot_max_y,
            //     ],
            // ];
            // self.functions[current_function]
            //     .current_plot_vec
            //     .insert(String::from("lines"), lines);

            egui_plot::Plot::new("My Plot")
                .legend(egui_plot::Legend::default())
                .show(ui, |plot_ui| {
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
                                .get("lines")
                            {
                                Some(lines) => lines.clone(),
                                None => vec![],
                            },
                        ))
                        .name("line"),
                    );
                });
        });
    }
}

fn get_root_chord_method(function: &Function) -> f64 {
    let fixed: f64;
    let mut x1: f64;
    let mut x2: f64;
    if function.f(function.a) * function.f_der2(function.a) > 0. {
        fixed = function.a;
        x1 = function.b;
        // println!("{}, {}", fixed, x1);
        x2 = x1 - (function.f(x1) / (function.f(x1) - function.f(fixed))) * (x1 - fixed);
        while (x2 - x1).abs() > function.eps && x2 <= function.b && x2 >= function.a {
            // println!("{}", (x2 - x1).abs());
            x1 = x2;
            x2 = x1 - (function.f(x1) / (function.f(fixed) - function.f(x1))) * (fixed - x1);
        }
    } else {
        fixed = function.a;
        x1 = function.b;
        x2 = x1 - (function.f(x1) / (function.f(fixed) - function.f(x1))) * (fixed - x1);
        // println!("{}, {}, {}", fixed, x1, x2);
        while (x2 - x1).abs() > function.eps && x2 <= function.b && x2 >= function.a {
            // println!("{}", (x2 - x1).abs());
            x1 = x2;
            x2 = x1 - (function.f(x1) / (function.f(fixed) - function.f(x1))) * (fixed - x1);
        }
    }

    // println!("{}", x2);
    x2
}
