use chrono::{DateTime, Local, TimeDelta};
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use egui_plot::{Line, Plot, PlotPoints};

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

    pub fn ui(
        ui: &mut egui::Ui,
        data: &Arc<RwLock<BTreeMap<DateTime<Local>, f64>>>,
        tspan: TimeDelta,
    ) {
        let now = Local::now();

        // TODO: agregar manejo de errores
        #[expect(clippy::unwrap_used)]
        let data = data.read().unwrap();
        let (last_tstamp, _last_sample) = data.last_key_value().unwrap_or((&now, &0.0));
        let first_tstamp = *last_tstamp - tspan;

        let points: PlotPoints<'_> = data
            .iter()
            .filter(|&(&tstamp, _)| tstamp > first_tstamp)
            .map(|(tstamp, value)| [tstamp.timestamp_micros() as f64, *value])
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
