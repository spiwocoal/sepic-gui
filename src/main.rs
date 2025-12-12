#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use log::error;
use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender};
use tokio::runtime::Runtime;

use sepic_gui::threading::{MessagingThread, ThreadMessage};

fn main() -> eframe::Result {
    let rt = Runtime::new().expect("No se pudo crear el Runtime para Tokio");
    let _enter = rt.enter();

    let (tx1, rx1): (Sender<ThreadMessage>, Receiver<ThreadMessage>) = mpsc::channel();
    let (tx2, rx2): (Sender<ThreadMessage>, Receiver<ThreadMessage>) = mpsc::channel();

    std::thread::Builder::new()
        .name("async_thread".to_owned())
        .spawn(move || -> ! {
            let mut thread_state = MessagingThread::new(rx1, tx2);

            rt.block_on(async move {
                loop {
                    thread_state.poll_messages().await.unwrap_or_else(|e| {
                        error!("Error en la comunicación entre hilos: {e}");
                    });
                }
            })
        })
        .expect("Error al crear el hilo para comunicación");

    let egui_logger = Box::new(egui_logger::builder().show_all_categories(false).build());
    let env_logger = Box::new(env_logger::builder().default_format().build());

    multi_log::MultiLogger::init(vec![egui_logger, env_logger], log::Level::Debug)
        .expect("Ocurrió un error al inicializar el gestor de registros");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 600.0])
            .with_min_inner_size([700.0, 400.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/ferris.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "SEPIC - Grupo 1 - Taller de Sistemas Electrónicos",
        native_options,
        Box::new(move |cc| Ok(Box::new(sepic_gui::SepicApp::new(cc, tx1, rx2)))),
    )
}
