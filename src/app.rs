use std::rc::Rc;

use crate::{PWMPreview, serialcomms::get_serial_ports};
use log::{debug, error};
use serialport::{SerialPort, SerialPortInfo};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct SepicApp {
    available_ports: Vec<Rc<SerialPortInfo>>,
    serial_port: Option<Box<dyn SerialPort>>,
    baudrate: u32,
    port_info: Option<Rc<SerialPortInfo>>,
    duty_cycle: f32,
    frequency: f32,
}

impl Default for SepicApp {
    fn default() -> Self {
        Self {
            available_ports: get_serial_ports(),
            serial_port: None,
            baudrate: 9600,
            port_info: None,
            duty_cycle: 1.0,
            frequency: 60e3,
        }
    }
}

impl SepicApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn update_serial_ports(&mut self) {
        self.available_ports = get_serial_ports();
    }
}

impl eframe::App for SepicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Archivo", |ui| {
                    if ui.button("Salir").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SEPIC");

            let prev_port = self.port_info.clone();

            ui.add(PWMPreview::new(self.duty_cycle, 4));

            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("⟲")).clicked() {
                    self.update_serial_ports();
                }
                egui::containers::ComboBox::from_label("Puerto serial")
                    .selected_text(if let Some(port) = &prev_port {
                        port.port_name.clone()
                    } else {
                        "Selecciona...".to_owned()
                    })
                    .show_ui(ui, |ui| {
                        ui.label("Selecciona...");
                        for port in &self.available_ports {
                            let port = Rc::clone(port);
                            ui.selectable_value(
                                &mut self.port_info,
                                Some(Rc::clone(&port)),
                                &port.port_name,
                            );
                        }
                    });

                egui::containers::ComboBox::from_label("Baudrate")
                    .selected_text(format!("{}", self.baudrate))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.baudrate, 9600, format!("{}", 9600));
                        ui.selectable_value(&mut self.baudrate, 38400, format!("{}", 38400));
                        ui.selectable_value(&mut self.baudrate, 115200, format!("{}", 115200));
                    });

                if prev_port != self.port_info
                    && let Some(port) = &self.port_info
                {
                    debug!("Se seleccionó nuevo puerto serial ({})", port.port_name);
                    self.serial_port = serialport::new(&port.port_name, self.baudrate)
                        .open()
                        .map_or_else(
                            |e| {
                                // TODO: mostrar un popup en GUI si falló la conexión al puerto
                                error!("No se pudo abrir el puerto {}: {:?}", port.port_name, e);
                                None
                            },
                            Some,
                        );
                }
            });

            let mut ui_builder = egui::UiBuilder::new();
            if self.serial_port.is_none() {
                ui_builder = ui_builder.disabled();
            }

            let prev_duty = self.duty_cycle;
            let prev_freq = self.frequency;

            ui.scope_builder(ui_builder, |ui| {
                ui.add(
                    egui::Slider::new(&mut self.duty_cycle, 0.0..=75.0)
                        .text("(%) Duty cycle")
                        .step_by(0.1)
                        .custom_formatter(|n, _| format!("{n:02.1}")),
                );
                ui.add(
                    egui::Slider::new(&mut self.frequency, 50e3..=100e3)
                        .text("(kHz) Frecuencia")
                        .custom_formatter(|n, _| {
                            let n = n / 1e3;
                            format!("{n:02.1}")
                        })
                        .custom_parser(|s| s.parse::<f64>().map(|n| n * 1000.0).ok()),
                );
            });

            if prev_duty != self.duty_cycle || prev_freq != self.frequency {
                debug!(
                    "Cambio en los parámetros PWM: ({},{}) -> ({},{})",
                    prev_freq, prev_duty, self.frequency, self.duty_cycle
                );
            }
        });
    }
}
