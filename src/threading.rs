use std::{
    io,
    net::{IpAddr, SocketAddr},
    str::FromStr as _,
    sync::mpsc::{Receiver, Sender, TryRecvError},
};

use anyhow::Result;
use log::debug;
use tokio::net::UdpSocket;

use crate::tabs::Measurement;

pub enum ThreadMessage {
    StartConnection { address: String, port: u16 },
    Disconnect,
    Data(Measurement),
    ConnectionEstablished,
    None,
}

struct Connection {
    #[expect(unused)]
    pub remote_addr: SocketAddr,
    pub socket: UdpSocket,
}

impl Connection {
    pub fn new(remote_addr: SocketAddr, socket: UdpSocket) -> Self {
        Self {
            remote_addr,
            socket,
        }
    }
}

pub struct MessagingThread {
    rx: Receiver<ThreadMessage>,
    tx: Sender<ThreadMessage>,

    connection: Option<Connection>,
}

impl MessagingThread {
    pub fn new(rx: Receiver<ThreadMessage>, tx: Sender<ThreadMessage>) -> Self {
        Self {
            rx,
            tx,
            connection: None,
        }
    }

    #[expect(clippy::missing_errors_doc)]
    pub async fn poll_messages(&mut self) -> Result<()> {
        if let Some(connection) = &self.connection {
            connection.socket.readable().await?;

            let mut buf = [0; 1024];
            match connection.socket.try_recv(&mut buf) {
                Ok(n) => {
                    let data = str::from_utf8(&buf)?;
                    debug!("Recibidos {n} bytes desde el dispositivo: {data}",);

                    let data = Measurement::from_str(data)?;
                    self.tx.send(ThreadMessage::Data(data))?;
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        let mut message = ThreadMessage::None;

        match self.rx.try_recv() {
            Ok(msg) => message = msg,
            Err(e) => {
                if matches!(e, TryRecvError::Disconnected) {
                    return Err(e.into());
                }
            }
        }

        match message {
            ThreadMessage::StartConnection { address, port } => {
                debug!(
                    "StartConnection recibido por hilo auxiliar con parámetros {address}:{port}",
                );

                let remote_addr = SocketAddr::new(IpAddr::from_str(address.as_str())?, port);

                let sock = UdpSocket::bind(remote_addr).await?;
                debug!("Socket creado {sock:?}");

                sock.send_to(b"START", remote_addr).await?;

                self.connection = Some(Connection::new(remote_addr, sock));
                self.tx.send(ThreadMessage::ConnectionEstablished)?;
            }
            ThreadMessage::Disconnect => {
                self.connection = None;
            }
            _ => {}
        }

        Ok(())
    }
}
