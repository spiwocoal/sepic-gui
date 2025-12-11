use std::{
    collections::BTreeMap,
    rc::Rc,
    sync::{Arc, RwLock},
};

mod pwm_plot;
use chrono::{DateTime, Local, TimeDelta};
use pwm_plot::PWMPlot;

mod meas_plot;
use meas_plot::MeasPlot;

mod logger;
use logger::LogConsole;

pub struct MyTabViewer {}

impl MyTabViewer {
    #[expect(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
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
            MyTab::PWMPlot {
                frequency,
                duty_cycle,
                tspan,
            } => PWMPlot::ui(ui, **frequency, **duty_cycle, *tspan),
            MyTab::MeasPlot { data, tspan } => MeasPlot::ui(ui, data, *tspan),
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
        frequency: Rc<f32>,
        duty_cycle: Rc<f32>,
        tspan: f64,
    },
    MeasPlot {
        data: Arc<RwLock<BTreeMap<DateTime<Local>, f64>>>,
        tspan: TimeDelta,
    },
    LogConsole,
}

impl MyTab {
    pub fn pwm_window(frequency: Rc<f32>, duty_cycle: Rc<f32>, tspan: f64) -> Self {
        Self::PWMPlot {
            frequency,
            duty_cycle,
            tspan,
        }
    }

    pub fn meas_window(
        data: Arc<RwLock<BTreeMap<DateTime<Local>, f64>>>,
        tspan: TimeDelta,
    ) -> Self {
        Self::MeasPlot { data, tspan }
    }

    pub fn log_window() -> Self {
        Self::LogConsole
    }
}
