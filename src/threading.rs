use std::sync::mpsc::{Receiver, Sender};

use chrono::{DateTime, Local};
use log::debug;
use tokio::net::UdpSocket;

pub enum ThreadMessage {
    StartConnection { address: String, port: u16 },
    Data { tstamp: DateTime<Local>, value: f64 },
    Ack,
}

pub async fn thread_messaging(rx: &Receiver<ThreadMessage>, tx: &Sender<ThreadMessage>) {
    // TODO: agregar manejo de errores
    #[expect(clippy::unwrap_used)]
    let message = rx.recv().unwrap();

    match message {
        ThreadMessage::StartConnection { address, port } => {
            // TODO: agregar manejo de errores
            #[expect(clippy::unwrap_used)]
            let sock = UdpSocket::bind((address.as_str(), port)).await.unwrap();
            #[expect(clippy::unwrap_used)]
            sock.send("START".as_bytes()).await.unwrap();
            debug!(
                "StartConnection recibido por hilo auxiliar con parámetros {}:{}",
                address, port
            );
            debug!("Socket creado {:?}", sock);
            tx.send(ThreadMessage::Ack).unwrap();
        }
        _ => {}
    }
}
