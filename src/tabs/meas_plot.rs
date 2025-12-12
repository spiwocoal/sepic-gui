use anyhow::anyhow;
use chrono::{DateTime, Local, TimeDelta};
use std::{cell::RefCell, collections::VecDeque, rc::Rc, str::FromStr};

use egui_plot::{Line, Plot, PlotPoints};

pub struct Measurement {
    pub timestamp: DateTime<Local>,
    pub value: f64,
}

impl Default for Measurement {
    fn default() -> Self {
        Self {
            timestamp: Local::now(),
            value: 0.0,
        }
    }
}

impl FromStr for Measurement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (tstamp, value) = s.split_once(' ').ok_or(anyhow!("ParseMeasurementError"))?;

        let tstamp_fromstr = tstamp
            .parse::<DateTime<Local>>()
            .map_err(|e| anyhow!("ParseMeasurementError: {e}"))?;
        let value_fromstr = value
            .parse::<f64>()
            .map_err(|e| anyhow!("ParseMeasurementError: {e}"))?;

        Ok(Self {
            timestamp: tstamp_fromstr,
            value: value_fromstr,
        })
    }
}

pub struct MeasPlot;

impl MeasPlot {
    // TODO: agregar cuadrícula con timestamps para gráfica
    #[expect(unused)]
    const MICROSECS_PER_MIN: f64 = 60.0 * 1e6;
    #[expect(unused)]
    const MICROSECS_PER_SEC: f64 = 1e6;
    #[expect(unused)]
    const MICROSECS_PER_MILLISEC: f64 = 1e3;

    pub fn title() -> egui::WidgetText {
        "Monitor de salida".into()
    }

    pub fn ui(ui: &mut egui::Ui, data: &Rc<RefCell<VecDeque<Measurement>>>, tspan: TimeDelta) {
        let fallback_measurement = Measurement::default();
        let data = data.borrow();

        let last_measurement = data.back().unwrap_or(&fallback_measurement);
        let first_tstamp = last_measurement.timestamp - tspan;

        let points: PlotPoints<'_> = data
            .iter()
            .filter(|&measurement| measurement.timestamp > first_tstamp)
            .map(|measurement| {
                [
                    measurement.timestamp.timestamp_micros() as f64,
                    measurement.value,
                ]
            })
            .collect();

        let line = Line::new("vo", points);

        Plot::new("meas_plot")
            .allow_scroll(true)
            .allow_zoom(true)
            .allow_boxed_zoom(true)
            .allow_drag(true)
            .include_y(50.0)
            .include_y(0.0)
            .x_axis_label("Tiempo / s")
            .y_axis_label("Voltaje / V")
            .show(ui, |plot_ui| plot_ui.line(line));
    }
}
