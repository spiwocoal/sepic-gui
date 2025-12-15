use std::{cell::RefCell, rc::Rc};

mod pwm_plot;
use chrono::TimeDelta;
use pwm_plot::PWMPlot;

mod meas_plot;
use meas_plot::MeasPlot;
pub use meas_plot::{Measurement, MeasurementRaw, Samples};

mod logger;
use logger::LogConsole;

pub struct MyTabViewer {
    frequency: f32,
    duty_cycle: f32,
    resistor_1: f64,
    resistor_2: f64,
}

impl MyTabViewer {
    #[expect(clippy::new_without_default)]
    pub fn new(frequency: f32, duty_cycle: f32, resistor_1: f64, resistor_2: f64) -> Self {
        Self {
            frequency,
            duty_cycle,
            resistor_1,
            resistor_2,
        }
    }
}

impl egui_dock::TabViewer for MyTabViewer {
    type Tab = MyTab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            MyTab::PWMPlot { .. } => PWMPlot::title(),
            MyTab::MeasPlot { .. } => MeasPlot::title(),
            MyTab::LogConsole => LogConsole::title(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            MyTab::PWMPlot { tspan } => PWMPlot::ui(ui, self.frequency, self.duty_cycle, *tspan),
            MyTab::MeasPlot { data, tspan } => {
                MeasPlot::ui(ui, data, self.resistor_1, self.resistor_2, *tspan)
            }
            MyTab::LogConsole => LogConsole::ui(ui),
        }
    }

    fn closeable(&mut self, tab: &mut Self::Tab) -> bool {
        match tab {
            MyTab::PWMPlot { .. } | MyTab::MeasPlot { .. } => false,
            MyTab::LogConsole => true,
        }
    }
}

pub enum MyTab {
    PWMPlot {
        tspan: f64,
    },
    MeasPlot {
        data: Rc<RefCell<Samples>>,
        tspan: TimeDelta,
    },
    LogConsole,
}

impl MyTab {
    pub fn pwm_window(tspan: f64) -> Self {
        Self::PWMPlot { tspan }
    }

    pub fn meas_window(data: Rc<RefCell<Samples>>, tspan: TimeDelta) -> Self {
        Self::MeasPlot { data, tspan }
    }

    pub fn log_window() -> Self {
        Self::LogConsole
    }
}
