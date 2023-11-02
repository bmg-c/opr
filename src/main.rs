use eframe::egui::{self, epaint, style, Color32};
use std::collections::HashMap;

pub fn set_theme(ctx: &egui::Context, theme: Theme) {
    let old = ctx.style().visuals.clone();
    ctx.set_visuals(theme.visuals(old));
}

pub fn set_style_theme(style: &mut egui::Style, theme: Theme) {
    let old = style.visuals.clone();
    style.visuals = theme.visuals(old);
}

fn make_widget_visual(
    old: style::WidgetVisuals,
    theme: &Theme,
    bg_fill: egui::Color32,
) -> style::WidgetVisuals {
    style::WidgetVisuals {
        bg_fill,
        weak_bg_fill: bg_fill,
        bg_stroke: egui::Stroke {
            color: theme.overlay1,
            ..old.bg_stroke
        },
        fg_stroke: egui::Stroke {
            color: theme.text,
            ..old.fg_stroke
        },
        ..old
    }
}

/// The colors for a theme variant.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Theme {
    pub rosewater: Color32,
    pub flamingo: Color32,
    pub pink: Color32,
    pub mauve: Color32,
    pub red: Color32,
    pub maroon: Color32,
    pub peach: Color32,
    pub yellow: Color32,
    pub green: Color32,
    pub teal: Color32,
    pub sky: Color32,
    pub sapphire: Color32,
    pub blue: Color32,
    pub lavender: Color32,
    pub text: Color32,
    pub subtext1: Color32,
    pub subtext0: Color32,
    pub overlay2: Color32,
    pub overlay1: Color32,
    pub overlay0: Color32,
    pub surface2: Color32,
    pub surface1: Color32,
    pub surface0: Color32,
    pub base: Color32,
    pub mantle: Color32,
    pub crust: Color32,
}

impl Theme {
    fn visuals(&self, old: egui::Visuals) -> egui::Visuals {
        let is_latte = *self == LATTE;
        egui::Visuals {
            override_text_color: Some(self.text),
            hyperlink_color: self.rosewater,
            faint_bg_color: self.surface0,
            extreme_bg_color: self.crust,
            code_bg_color: self.mantle,
            warn_fg_color: self.peach,
            error_fg_color: self.maroon,
            window_fill: self.base,
            panel_fill: self.base,
            window_stroke: egui::Stroke {
                color: self.overlay1,
                ..old.window_stroke
            },
            widgets: style::Widgets {
                noninteractive: make_widget_visual(old.widgets.noninteractive, self, self.base),
                inactive: make_widget_visual(old.widgets.inactive, self, self.surface0),
                hovered: make_widget_visual(old.widgets.hovered, self, self.surface2),
                active: make_widget_visual(old.widgets.active, self, self.surface1),
                open: make_widget_visual(old.widgets.open, self, self.surface0),
            },
            selection: style::Selection {
                bg_fill: self.blue.linear_multiply(if is_latte { 0.4 } else { 0.2 }),
                stroke: egui::Stroke {
                    color: self.overlay1,
                    ..old.selection.stroke
                },
            },
            window_shadow: epaint::Shadow {
                color: self.base,
                ..old.window_shadow
            },
            popup_shadow: epaint::Shadow {
                color: self.base,
                ..old.popup_shadow
            },
            dark_mode: !is_latte,
            ..old
        }
    }
}

pub const LATTE: Theme = Theme {
    rosewater: Color32::from_rgb(220, 138, 120),
    flamingo: Color32::from_rgb(221, 120, 120),
    pink: Color32::from_rgb(234, 118, 203),
    mauve: Color32::from_rgb(136, 57, 239),
    red: Color32::from_rgb(210, 15, 57),
    maroon: Color32::from_rgb(230, 69, 83),
    peach: Color32::from_rgb(254, 100, 11),
    yellow: Color32::from_rgb(223, 142, 29),
    green: Color32::from_rgb(64, 160, 43),
    teal: Color32::from_rgb(23, 146, 153),
    sky: Color32::from_rgb(4, 165, 229),
    sapphire: Color32::from_rgb(32, 159, 181),
    blue: Color32::from_rgb(30, 102, 245),
    lavender: Color32::from_rgb(114, 135, 253),
    text: Color32::from_rgb(76, 79, 105),
    subtext1: Color32::from_rgb(92, 95, 119),
    subtext0: Color32::from_rgb(108, 111, 133),
    overlay2: Color32::from_rgb(124, 127, 147),
    overlay1: Color32::from_rgb(140, 143, 161),
    overlay0: Color32::from_rgb(156, 160, 176),
    surface2: Color32::from_rgb(172, 176, 190),
    surface1: Color32::from_rgb(188, 192, 204),
    surface0: Color32::from_rgb(204, 208, 218),
    base: Color32::from_rgb(239, 241, 245),
    mantle: Color32::from_rgb(230, 233, 239),
    crust: Color32::from_rgb(220, 224, 232),
};

pub const FRAPPE: Theme = Theme {
    rosewater: Color32::from_rgb(242, 213, 207),
    flamingo: Color32::from_rgb(238, 190, 190),
    pink: Color32::from_rgb(244, 184, 228),
    mauve: Color32::from_rgb(202, 158, 230),
    red: Color32::from_rgb(231, 130, 132),
    maroon: Color32::from_rgb(234, 153, 156),
    peach: Color32::from_rgb(239, 159, 118),
    yellow: Color32::from_rgb(229, 200, 144),
    green: Color32::from_rgb(166, 209, 137),
    teal: Color32::from_rgb(129, 200, 190),
    sky: Color32::from_rgb(153, 209, 219),
    sapphire: Color32::from_rgb(133, 193, 220),
    blue: Color32::from_rgb(140, 170, 238),
    lavender: Color32::from_rgb(186, 187, 241),
    text: Color32::from_rgb(198, 208, 245),
    subtext1: Color32::from_rgb(181, 191, 226),
    subtext0: Color32::from_rgb(165, 173, 206),
    overlay2: Color32::from_rgb(148, 156, 187),
    overlay1: Color32::from_rgb(131, 139, 167),
    overlay0: Color32::from_rgb(115, 121, 148),
    surface2: Color32::from_rgb(98, 104, 128),
    surface1: Color32::from_rgb(81, 87, 109),
    surface0: Color32::from_rgb(65, 69, 89),
    base: Color32::from_rgb(48, 52, 70),
    mantle: Color32::from_rgb(41, 44, 60),
    crust: Color32::from_rgb(35, 38, 52),
};

pub const MACCHIATO: Theme = Theme {
    rosewater: Color32::from_rgb(244, 219, 214),
    flamingo: Color32::from_rgb(240, 198, 198),
    pink: Color32::from_rgb(245, 189, 230),
    mauve: Color32::from_rgb(198, 160, 246),
    red: Color32::from_rgb(237, 135, 150),
    maroon: Color32::from_rgb(238, 153, 160),
    peach: Color32::from_rgb(245, 169, 127),
    yellow: Color32::from_rgb(238, 212, 159),
    green: Color32::from_rgb(166, 218, 149),
    teal: Color32::from_rgb(139, 213, 202),
    sky: Color32::from_rgb(145, 215, 227),
    sapphire: Color32::from_rgb(125, 196, 228),
    blue: Color32::from_rgb(138, 173, 244),
    lavender: Color32::from_rgb(183, 189, 248),
    text: Color32::from_rgb(202, 211, 245),
    subtext1: Color32::from_rgb(184, 192, 224),
    subtext0: Color32::from_rgb(165, 173, 203),
    overlay2: Color32::from_rgb(147, 154, 183),
    overlay1: Color32::from_rgb(128, 135, 162),
    overlay0: Color32::from_rgb(110, 115, 141),
    surface2: Color32::from_rgb(91, 96, 120),
    surface1: Color32::from_rgb(73, 77, 100),
    surface0: Color32::from_rgb(54, 58, 79),
    base: Color32::from_rgb(36, 39, 58),
    mantle: Color32::from_rgb(30, 32, 48),
    crust: Color32::from_rgb(24, 25, 38),
};

pub const MOCHA: Theme = Theme {
    rosewater: Color32::from_rgb(245, 224, 220),
    flamingo: Color32::from_rgb(242, 205, 205),
    pink: Color32::from_rgb(245, 194, 231),
    mauve: Color32::from_rgb(203, 166, 247),
    red: Color32::from_rgb(243, 139, 168),
    maroon: Color32::from_rgb(235, 160, 172),
    peach: Color32::from_rgb(250, 179, 135),
    yellow: Color32::from_rgb(249, 226, 175),
    green: Color32::from_rgb(166, 227, 161),
    teal: Color32::from_rgb(148, 226, 213),
    sky: Color32::from_rgb(137, 220, 235),
    sapphire: Color32::from_rgb(116, 199, 236),
    blue: Color32::from_rgb(137, 180, 250),
    lavender: Color32::from_rgb(180, 190, 254),
    text: Color32::from_rgb(205, 214, 244),
    subtext1: Color32::from_rgb(186, 194, 222),
    subtext0: Color32::from_rgb(166, 173, 200),
    overlay2: Color32::from_rgb(147, 153, 178),
    overlay1: Color32::from_rgb(127, 132, 156),
    overlay0: Color32::from_rgb(108, 112, 134),
    surface2: Color32::from_rgb(88, 91, 112),
    surface1: Color32::from_rgb(69, 71, 90),
    surface0: Color32::from_rgb(49, 50, 68),
    base: Color32::from_rgb(30, 30, 46),
    mantle: Color32::from_rgb(24, 24, 37),
    crust: Color32::from_rgb(17, 17, 27),
};

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
        // use eframe::{
        //     egui::{FontFamily, TextStyle},
        //     epaint::FontId,
        // };
        // use FontFamily::{Monospace, Proportional};
        // let mut style = (*cc.egui_ctx.style()).clone();
        // style.text_styles = [
        //     (TextStyle::Heading, FontId::new(25.0, Proportional)),
        //     (
        //         TextStyle::Name("Heading2".into()),
        //         FontId::new(22.0, Proportional),
        //     ),
        //     (
        //         TextStyle::Name("ContextHeading".into()),
        //         FontId::new(19.0, Proportional),
        //     ),
        //     (TextStyle::Body, FontId::new(16.0, Proportional)),
        //     (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        //     (TextStyle::Button, FontId::new(16.0, Proportional)),
        //     (TextStyle::Small, FontId::new(20.0, Proportional)),
        // ]
        // .into();
        // cc.egui_ctx.set_style(style);
        MathApp {
            current_function: CurrentFunction::First,
            functions: vec![
                Function::new(
                    "exp(-x) * cos(x * PI)",
                    -1.0,
                    1.0,
                    0.001,
                    CurrentFunction::First,
                ),
                Function::new(
                    "3x^4 - 4x^3 - 12x^2 + 2",
                    -1.0,
                    1.0,
                    0.001,
                    CurrentFunction::Second,
                ),
                Function::new("x^2 - 5sin(x)", -1.0, 1.0, 0.001, CurrentFunction::Third),
                Function::new("0.1x^2 - xln(x)", -1.0, 1.0, 0.001, CurrentFunction::Fourth),
            ],
            theme: LATTE,
        }
    }
}

impl eframe::App for MathApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        set_theme(ctx, self.theme);
        let current_function: usize = match self.current_function {
            CurrentFunction::First => 0,
            CurrentFunction::Second => 1,
            CurrentFunction::Third => 2,
            CurrentFunction::Fourth => 3,
        };
        egui::TopBottomPanel::top("Function select").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    ui.heading("Hord method. ");
                    ui.label("Source code:");
                    ui.hyperlink_to("🔌 GitHub repo", "https://github.com/bmg-c/opr");
                });
                ui.with_layout(
                    egui::Layout::right_to_left(eframe::emath::Align::RIGHT),
                    |ui| {
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
            ui.horizontal(|ui| {
                ui.heading(format!(
                    "x = {}, f(x) = {}",
                    self.functions[current_function].x2,
                    self.functions[current_function].f(self.functions[current_function].x2)
                ));
                ui.with_layout(
                    egui::Layout::right_to_left(eframe::emath::Align::RIGHT),
                    |ui| {
                        ui.heading(format!(
                            "Iterations: {}",
                            self.functions[current_function].current_iteration
                        ));
                    },
                );
            });
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
            ui.horizontal(|ui| {
                ui.label("Choose your equation: ");
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
            // ui.heading(format!("{} = 0", &self.functions[current_function].title));
            ui.horizontal(|ui| {
                ui.label("Set data:");
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
                        .clamp_range(0..=i64::MAX)
                        .prefix("eps: "),
                );
                ui.label("Apply data:");
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

            // ui.horizontal(|ui| {
            egui_plot::Plot::new("My Plot")
                .legend(egui_plot::Legend::default())
                .show(ui, |plot_ui| {
                    // let mut x_vec: [f64; 2] = [0.0, 0.0];
                    // if match self.functions[current_function]
                    //     .current_plot_vec
                    //     .get("lines_x2")
                    // {
                    //     Some(lines) => {
                    //         if lines.len() != 0 {
                    //             x_vec = lines[1];
                    //         }
                    //         true
                    //     }
                    //     None => false,
                    // } {
                    //     plot_ui.text(
                    //         egui_plot::Text::new(
                    //             egui_plot::PlotPoint::from(x_vec),
                    //             format!("{}", self.functions[current_function].x2),
                    //         )
                    //         .color(self.theme.text)
                    //         .name("x"),
                    //     );
                    // }
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
                        .color(self.theme.red)
                        .name(self.functions[current_function].title.as_str()),
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
                        .color(self.theme.teal)
                        .name("Showcase"),
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
                        .color(self.theme.green)
                        .name("Showcase"),
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
                        .color(self.theme.pink)
                        .name("Left and Right borders"),
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
                        .color(self.theme.pink)
                        .name("Left and Right borders"),
                    );
                });
            // ui.label("");
            // });
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
                                let lines: Vec<[f64; 2]> = match self.functions[current_function]
                                    .current_plot_vec
                                    .get("lines_x2")
                                {
                                    Some(lines) => lines.clone(),
                                    None => vec![],
                                };
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
