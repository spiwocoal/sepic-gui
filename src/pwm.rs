use egui::{
    Color32, Frame, Pos2, Rect, Response, Stroke, Ui, Widget, emath, epaint::PathShape, pos2, vec2,
};
use log::debug;

pub struct PWMPreview {
    duty_cycle: f32,
    n_periods: u8,
}

impl PWMPreview {
    pub fn new(duty_cycle: f32, n_periods: u8) -> Self {
        Self {
            duty_cycle,
            n_periods,
        }
    }
}

impl Widget for PWMPreview {
    fn ui(self, ui: &mut Ui) -> Response {
        Frame::canvas(ui.style()).show(ui, |ui| {
            ui.ctx().request_repaint();
            let desired_size = ui.available_width() * vec2(1.0, 0.7);
            let (_id, rect) = ui.allocate_space(desired_size);

            let to_screen =
                emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);

            let n = 2 * self.n_periods;

            // let mut points: Vec<Pos2> = (0..=n)
            //     .map(|i| {
            //         if i % 4 < 3 {
            //             if i % 2 == 0 {
            //                 to_screen * pos2(i as f32 / n as f32, 0.25)
            //             } else {
            //                 to_screen
            //                     * pos2(
            //                         (i as f32 + 1.0 - (self.duty_cycle / 100.0)) / n as f32,
            //                         0.25,
            //                     )
            //             }
            //         } else if i % 2 == 0 {
            //             to_screen * pos2((i - 1) as f32 / n as f32, 0.75)
            //         } else {
            //             to_screen
            //                 * pos2(((i - 1) as f32 + self.duty_cycle / 100.0) / n as f32, 0.75)
            //         }
            //         // to_screen
            //         //     * pos2(
            //         //         (i % 2) as f32 * ((1.0 - self.duty_cycle) / 100.0)
            //         //             + i as f32 / n as f32,
            //         //         0.25 + (i % 2) as f32 * 0.5,
            //         //     )
            //     })
            //     .collect();
            let mut points = vec![];
            for i in 0..n {
                if i % 2 == 0 {
                    points.push(to_screen * pos2(i as f32 / n as f32, 0.25));
                    points.push(
                        to_screen
                            * pos2(
                                (i + 1) as f32 + 1.0 - (self.duty_cycle / 100.0) / n as f32,
                                0.25,
                            ),
                    );
                } else {
                    points.push(
                        to_screen
                            * pos2(i as f32 + 1.0 - (self.duty_cycle / 100.0) / n as f32, 0.75),
                    );
                    points.push(
                        to_screen
                            * pos2((i + 1) as f32 + (self.duty_cycle / 100.0) / n as f32, 0.75),
                    );
                }
            }
            let shape = PathShape::line(points, Stroke::new(1.0, Color32::LIGHT_GREEN));
            ui.painter().add(shape);
        });

        ui.response()
    }
}
