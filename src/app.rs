use std::{rc::Rc, time::Duration};

use crate::{
    MyTabViewer,
    serialcomms::{attempt_handshake, get_serial_ports, set_duty, set_frequency},
    tabs::MyTab,
};
use egui::Ui;
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use log::{debug, error};
use serialport::{SerialPort, SerialPortInfo};

pub struct SepicApp {
    available_ports: Vec<Rc<SerialPortInfo>>,
    serial_port: Option<Box<dyn SerialPort>>,
    baudrate: u32,
    port_info: Option<Rc<SerialPortInfo>>,
    duty_cycle: f32,
    frequency: f32,
    tree: DockState<MyTab>,
}

impl Default for SepicApp {
    fn default() -> Self {
        let mut tree = DockState::new(vec![MyTab::plot_window()]);
        let [_, _] =
            tree.main_surface_mut()
                .split_below(NodeIndex::root(), 0.75, vec![MyTab::log_window()]);

        Self {
            available_ports: get_serial_ports(),
            serial_port: None,
            baudrate: 9600,
            port_info: None,
            duty_cycle: 0.0,
            frequency: 60e3,
            tree,
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

impl SepicApp {
    fn update_menubar(ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
    }

    fn update_settingsbar(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let prev_duty = self.duty_cycle;
        let prev_freq = self.frequency;

        egui::SidePanel::left("Ajustes").show(ctx, |ui| {
            ui.heading("SEPIC");
            ui.vertical(|ui| {
                self.update_serial_settings(ui);

                let mut ui_builder = egui::UiBuilder::new();
                if self.serial_port.is_none() {
                    ui_builder = ui_builder.disabled();
                }

                ui.scope_builder(ui_builder, |ui| {
                    ui.add(
                        egui::Slider::new(&mut self.duty_cycle, 0.0..=75.0)
                            .text("(%) Duty cycle")
                            .step_by(0.1)
                            .custom_formatter(|n, _| format!("{n:02.1}")),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.frequency, 60e3..=120e3)
                            .text("(kHz) Frecuencia")
                            .custom_formatter(|n, _| {
                                let n = n / 1e3;
                                format!("{n:02.1}")
                            })
                            .custom_parser(|s| s.parse::<f64>().map(|n| n * 1000.0).ok()),
                    );
                });
            });
        });

        if let Some(serial_port) = self.serial_port.as_mut() {
            if prev_duty != self.duty_cycle {
                debug!("Actualizando ciclo de trabajo a {}", self.duty_cycle);
                set_duty(serial_port, self.duty_cycle).unwrap_or_else(|e| {
                    error!("No se pudo actualizar el ciclo de trabajo: {e}");
                });
            }

            if prev_freq != self.frequency {
                debug!("Actualizando frecuencia a {}", self.frequency);
                set_frequency(serial_port, self.frequency)
                    .unwrap_or_else(|e| error!("No se pudo actualizar la frecuencia: {e}"));
            }
        }
    }

    fn update_serial_settings(&mut self, ui: &mut Ui) {
        let prev_port = self.port_info.clone();

        ui.collapsing("Conexión serial", |ui| {
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("⟲")).clicked() {
                    self.update_serial_ports();
                    debug!("Puerto seriales disponibles: {:?}", self.available_ports);
                    debug!("Puerto serial seleccionado: {:?}", self.port_info);
                    if let Some(port_info) = self.port_info.as_ref()
                        && !self.available_ports.contains(port_info)
                    {
                        self.port_info = None;
                    }
                }
                egui::containers::ComboBox::from_label("Puerto serial")
                    .selected_text(if let Some(port) = &prev_port {
                        port.port_name.clone()
                    } else {
                        "Selecciona...".to_owned()
                    })
                    .show_ui(ui, |ui| {
                        self.update_serial_ports();
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
            });

            egui::containers::ComboBox::from_label("Baudrate")
                .selected_text(format!("{}", self.baudrate))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.baudrate, 9600, format!("{}", 9600));
                    ui.selectable_value(&mut self.baudrate, 38400, format!("{}", 38400));
                    ui.selectable_value(&mut self.baudrate, 115200, format!("{}", 115200));
                });
        });

        if prev_port != self.port_info
            && let Some(port) = &self.port_info
        {
            debug!("Se seleccionó nuevo puerto serial `{}`", port.port_name);
            self.serial_port = serialport::new(&port.port_name, self.baudrate)
                .timeout(Duration::from_millis(500))
                .open()
                .map_or_else(
                    |e| {
                        // TODO: mostrar un popup en GUI si falló la conexión al puerto
                        error!("No se pudo abrir el puerto `{}`: `{:?}`", port.port_name, e);
                        None
                    },
                    |mut port| match attempt_handshake(&mut port) {
                        Ok(_) => Some(port),
                        Err(e) => {
                            error!("Falló el handshake con el dispositivo: {e:?}");
                            None
                        }
                    },
                );
        }
    }
}

impl eframe::App for SepicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Self::update_menubar(ctx, _frame);
        self.update_settingsbar(ctx, _frame);

        let mut viewer = MyTabViewer::new(self.frequency, self.duty_cycle, 4);

        DockArea::new(&mut self.tree)
            .style(Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut viewer);
    }
}
