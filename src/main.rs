#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> eframe::Result {
    let egui_logger = Box::new(egui_logger::builder().show_all_categories(false).build());
    let env_logger = Box::new(env_logger::builder().default_format().build());

    multi_log::MultiLogger::init(vec![egui_logger, env_logger], log::Level::Debug)
        .expect("Ocurrió un error al inicializar el gestor de registros");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 600.0])
            .with_min_inner_size([700.0, 400.0]), /*.with_icon(
                                                      eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                                                          .expect("Failed to load icon"),
                                                  )*/
        ..Default::default()
    };
    eframe::run_native(
        "SEPIC - Grupo 1 - Taller de Sistemas Electrónicos",
        native_options,
        Box::new(|cc| Ok(Box::new(sepic_gui::SepicApp::new(cc)))),
    )
}
