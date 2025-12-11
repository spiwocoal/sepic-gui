use crate::MyTabViewer;
use egui_plot::{Line, Plot, PlotPoints};

pub struct MeasPlot;

impl MeasPlot {
    pub fn id() -> &'static str {
        "meas_plot_window"
    }
    pub fn title() -> egui::WidgetText {
        "Voltaje de salida".into()
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
