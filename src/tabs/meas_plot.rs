use anyhow::anyhow;
use chrono::{DateTime, Local, TimeDelta, Timelike as _};
use std::{cell::RefCell, collections::VecDeque, ops::RangeInclusive, rc::Rc, str::FromStr};

use egui_plot::{GridInput, GridMark, Line, Plot, PlotPoints};

#[derive(Default)]
pub struct Samples {
    pub data: VecDeque<VecDeque<Measurement>>,
}

impl Samples {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pop_front(&mut self) {
        if let Some(front) = self.data.front_mut() {
            front.pop_front();
        }
    }

    pub fn push_back(&mut self, value: Measurement) {
        if let Some(back) = self.data.back_mut() {
            back.push_back(value);
        }
    }

    pub fn push_collection(&mut self) {
        self.data.push_back(VecDeque::new());
    }

    pub fn len(&self) -> usize {
        self.data.iter().map(|v| v.len()).sum()
    }

    pub fn back(&self) -> Option<&Measurement> {
        if let Some(back) = self.data.back() {
            return back.back();
        }

        return None;
    }
}

pub struct MeasurementRaw {
    pub timestamp: u32,
    pub value: f64,
}

impl FromStr for MeasurementRaw {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<_> = s
            .split(',')
            .map(|s| s.split_once(':').unwrap_or_default().1)
            .collect();

        let value_fromstr = values
            .first()
            .ok_or(anyhow!("ParseMeasurementError"))?
            .trim_ascii()
            .parse::<f64>()
            .map_err(|e| anyhow!("ParseMeasurementError: {e}"))?;

        let tstamp_fromstr = values
            .get(1)
            .ok_or(anyhow!("ParseMeasurementError"))?
            .trim_ascii()
            .parse::<u32>()
            .map_err(|e| anyhow!("ParseMeasurementError: {e}"))?;

        Ok(Self {
            timestamp: tstamp_fromstr,
            value: value_fromstr,
        })
    }
}

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

impl Measurement {
    pub fn new(timestamp_offset: u32, date_offset: DateTime<Local>, raw: &MeasurementRaw) -> Self {
        let delta_timestamp = raw.timestamp - timestamp_offset;
        let timestamp = date_offset + TimeDelta::microseconds(delta_timestamp.into());

        Self {
            timestamp,
            value: raw.value,
        }
    }
}

pub struct MeasPlot;

impl MeasPlot {
    const MILLISECS_PER_MIN: f64 = 60.0 * 1e3;
    const MILLISECS_PER_SEC: f64 = 1e3;

    pub fn title() -> egui::WidgetText {
        "Monitor de salida".into()
    }

    pub fn ui(
        ui: &mut egui::Ui,
        data: &Rc<RefCell<Samples>>,
        resistor_1: f64,
        resistor_2: f64,
        tspan: TimeDelta,
    ) {
        let fallback_collection = VecDeque::new();
        let fallback_measurement = Measurement::default();
        let data = &data.borrow().data;

        let last_measurement = data
            .back()
            .unwrap_or(&fallback_collection)
            .back()
            .unwrap_or(&fallback_measurement);
        let first_tstamp = last_measurement.timestamp - tspan;

        let divider_ratio = resistor_2 / (resistor_1 + resistor_2);

        let x_grid = |input: GridInput| {
            let mut marks: Vec<GridMark> = vec![];

            let (min, max) = input.bounds;
            let min = min.floor() as i64;
            let max = max.ceil() as i64;

            for i in min..=max {
                let tstamp = first_tstamp + TimeDelta::milliseconds(i);
                let step_size = if tstamp.second() == 0 && tstamp.timestamp_subsec_millis() == 0 {
                    Self::MILLISECS_PER_MIN
                } else if tstamp.second().is_multiple_of(10)
                    && tstamp.timestamp_subsec_millis() == 0
                {
                    Self::MILLISECS_PER_SEC
                } else {
                    continue;
                };

                marks.push(GridMark {
                    value: i as f64,
                    step_size,
                });
            }

            marks
        };

        let time_formatter = |mark: GridMark, _range: &RangeInclusive<f64>| {
            let milliseconds = mark.value;
            let tstamp = first_tstamp + TimeDelta::milliseconds(milliseconds as i64);
            format!("{}", tstamp.format("%H:%M"))
        };

        let lines: Vec<Line<'_>> = data
            .iter()
            .map(|d| {
                Line::new(
                    "vo",
                    d.iter()
                        .filter(|&measurement| measurement.timestamp > first_tstamp)
                        .map(|measurement| {
                            [
                                (last_measurement.timestamp - measurement.timestamp)
                                    .num_milliseconds() as f64,
                                measurement.value / divider_ratio,
                            ]
                        })
                        .collect::<PlotPoints<'_>>(),
                )
            })
            .collect();

        Plot::new("meas_plot")
            .x_grid_spacer(x_grid)
            .x_axis_formatter(time_formatter)
            .allow_scroll(true)
            .allow_zoom(true)
            .allow_boxed_zoom(true)
            .allow_drag(true)
            .auto_bounds([false, true])
            .include_y(50.0)
            .include_y(0.0)
            .include_x(-1.0)
            .include_x(tspan.num_milliseconds() as f64)
            .x_axis_label("Hora")
            .y_axis_label("Voltaje / V")
            .show(ui, |plot_ui| {
                for line in lines {
                    plot_ui.line(line);
                }
            });
    }
}
