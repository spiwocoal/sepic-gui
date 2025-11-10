use egui::{
    Color32, Frame, Pos2, Rect, Response, Shape, Stroke, Ui, Widget, emath, epaint::PathShape,
    pos2, vec2,
};

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
        let (path_color, dot_color) = if ui.visuals().dark_mode {
            (Color32::from_additive_luminance(196), Color32::WHITE)
        } else {
            (Color32::from_black_alpha(240), Color32::BLACK)
        };

        Frame::canvas(ui.style()).show(ui, |ui| {
            ui.ctx().request_repaint();
            let desired_size = ui.available_width() * vec2(0.65, 0.45);
            let (_id, rect) = ui.allocate_space(desired_size);

            let to_screen =
                emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, 1.0..=0.0), rect);

            let n = self.n_periods;

            let points: Vec<Pos2> = (0..n)
                .flat_map(|i| {
                    vec![
                        to_screen * pos2(i as f32 / n as f32, 0.25),
                        to_screen
                            * pos2(
                                (i as f32 + 1.0 - (self.duty_cycle / 100.0)) / n as f32,
                                0.25,
                            ),
                        to_screen
                            * pos2(
                                (i as f32 + 1.0 - (self.duty_cycle / 100.0)) / n as f32,
                                0.75,
                            ),
                        to_screen * pos2((i + 1) as f32 / n as f32, 0.75),
                    ]
                })
                .collect();

            let shape = PathShape::line(points.clone(), Stroke::new(1.0, path_color));
            ui.painter().add(shape);

            let shape: Vec<Shape> = points
                .into_iter()
                .map(|point| Shape::circle_filled(point, 2.0, dot_color))
                .collect();
            ui.painter().extend(shape);
        });

        ui.response()
    }
}
