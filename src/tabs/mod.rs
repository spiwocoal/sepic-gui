mod pwm_plot;
use pwm_plot::PWMPlot;

mod meas_plot;
use meas_plot::MeasPlot;

mod logger;
use logger::LogConsole;

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
            MyTab::PWMPlot(_) => PWMPlot::title(),
            MyTab::MeasPlot(_) => MeasPlot::title(),
            MyTab::LogConsole(_) => LogConsole::title(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            MyTab::PWMPlot(_) => PWMPlot::ui(ui, self),
            MyTab::MeasPlot(_) => MeasPlot::ui(ui, self),
            MyTab::LogConsole(_) => LogConsole::ui(ui),
        }
    }

    fn closeable(&mut self, tab: &mut Self::Tab) -> bool {
        match tab {
            MyTab::PWMPlot(_) | MyTab::MeasPlot(_) => false,
            MyTab::LogConsole(_) => true,
        }
    }
}

pub enum MyTab {
    PWMPlot(PWMPlot),
    MeasPlot(MeasPlot),
    LogConsole(LogConsole),
}

impl MyTab {
    pub fn pwm_window() -> Self {
        Self::PWMPlot(PWMPlot)
    }

    pub fn meas_window() -> Self {
        Self::MeasPlot(MeasPlot)
    }

    pub fn log_window() -> Self {
        Self::LogConsole(LogConsole)
    }
}
