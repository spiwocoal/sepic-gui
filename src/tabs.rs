use egui_plot::{Line, Plot, PlotPoints};

pub struct MyTabViewer {
    frequency: f32,
    duty_cycle: f32,
    tspan: f64,
}

impl MyTabViewer {
    pub fn new(frequency: f32, duty_cycle: f32, tspan: f64) -> Self {
        Self {
            frequency,
            duty_cycle,
            tspan,
        }
    }
}

impl egui_dock::TabViewer for MyTabViewer {
    type Tab = MyTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            MyTab::PlotWindow(_) => PlotWindow::title(),
            MyTab::LogConsole(_) => LogConsole::title(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            MyTab::PlotWindow(_) => PlotWindow::ui(ui, self),
            MyTab::LogConsole(_) => LogConsole::ui(ui),
        }
    }

    fn closeable(&mut self, tab: &mut Self::Tab) -> bool {
        match tab {
            MyTab::PlotWindow(_) => false,
            MyTab::LogConsole(_) => true,
        }
    }
}

pub enum MyTab {
    PlotWindow(PlotWindow),
    LogConsole(LogConsole),
}

impl MyTab {
    pub fn plot_window() -> Self {
        Self::PlotWindow(PlotWindow)
    }

    pub fn log_window() -> Self {
        Self::LogConsole(LogConsole)
    }
}

pub struct PlotWindow;

impl PlotWindow {
    pub fn id() -> &'static str {
        "pwm_plot_window"
    }
    pub fn title() -> egui::WidgetText {
        "Señal PWM".into()
    }

    pub fn ui(ui: &mut egui::Ui, viewer: &MyTabViewer) {
        let tspan = viewer.tspan;
        let duty_cycle = viewer.duty_cycle;
        let frequency = viewer.frequency;
        let period = (1.0 / frequency as f64) * 1e6;

        let points: PlotPoints<'_> = (0..10 * tspan as u32)
            .map(|i| {
                let t = (i as f64) / 10.0;
                let duty = duty_cycle as f64 / 100.0;
                [
                    t,
                    if (t % period) < period * (1.0 - duty) {
                        0.0
                    } else {
                        5.0
                    },
                ]
            })
            .collect();

        let line = Line::new("pwm", points);
        Plot::new("pwm_plot")
            .x_grid_spacer(|_| {
                (0..(4.0 * tspan / period) as u32)
                    .map(|i| {
                        let step = if i % 4 == 0 { 100.0 } else { 75.0 };
                        egui_plot::GridMark {
                            value: (i as f64) * period,
                            step_size: step,
                        }
                    })
                    .collect()
            })
            .allow_scroll(false)
            .allow_zoom(false)
            .allow_boxed_zoom(false)
            .allow_drag(false)
            .include_y(5.0)
            .include_y(0.0)
            .x_axis_label("Tiempo / μs")
            .y_axis_label("Voltaje / V")
            .show(ui, |plot_ui| plot_ui.line(line));
    }
}

pub struct LogConsole;

impl LogConsole {
    pub fn id() -> &'static str {
        "log_console"
    }
    pub fn title() -> egui::WidgetText {
        "Registros".into()
    }
    pub fn ui(ui: &mut egui::Ui) {
        egui_logger::logger_ui()
            .log_levels([true, true, true, true, false])
            .enable_ctx_menu(false)
            .enable_regex(false)
            .enable_max_log_output(false)
            .enable_category("sepic_gui::app", true)
            .enable_category("sepic_gui::serialcomms", true)
            .show(ui);
    }
}
