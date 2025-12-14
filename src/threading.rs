use std::{
    char,
    net::{IpAddr, SocketAddr},
    str::FromStr as _,
    sync::Arc,
    time::Duration,
};

use anyhow::{Result, anyhow};
use chrono::{DateTime, Local};
use log::{debug, error, trace};
use tokio::{
    net::UdpSocket,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    time::timeout,
};

type Receiver<T> = UnboundedReceiver<T>;
type Sender<T> = UnboundedSender<T>;

use crate::tabs::{Measurement, MeasurementRaw};

pub enum ThreadMessage {
    StartConnection {
        address: String,
        port: u16,
        ctx: Arc<egui::Context>,
    },
    Disconnect,
    Shutdown,
    Data(Measurement),
    ConnectionEstablished,
    None,
}

struct Connection {
    pub remote_addr: SocketAddr,
    pub socket: UdpSocket,
}

#[derive(Clone, Copy)]
struct DateOffset {
    pub timestamp: u32,
    pub date: DateTime<Local>,
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
    ctx: Option<Arc<egui::Context>>,

    connection: Option<Connection>,
    date_offset: Option<DateOffset>,
}

impl MessagingThread {
    pub fn new(rx: Receiver<ThreadMessage>, tx: Sender<ThreadMessage>) -> Self {
        Self {
            rx,
            tx,
            ctx: None,
            connection: None,
            date_offset: None,
        }
    }

    #[expect(clippy::missing_errors_doc)]
    pub async fn poll_messages(&mut self) -> Result<bool> {
        let mut buf = [0; 1024];

        tokio::select! {
                Some(n) = async {
                    if let Some(connection) = &self.connection {
                        if let Ok(res) = timeout(Duration::from_secs(5), connection.socket.recv(&mut buf)).await {
                            res.ok()
                        } else {
                            self.tx.send(ThreadMessage::Disconnect).unwrap_or_else(|_| {
                                error!("Ocurrió un problema al utilizar el canal");}
                            );
                            if let Some(ctx) = self.ctx.as_ref() {
                                ctx.request_repaint();
                            }
                            None
                        }
                    } else {
                        None
                    }
                } => {
                        let data = str::from_utf8(&buf)?.trim().trim_matches(char::from(0));
                        trace!("Recibidos {n} bytes desde el dispositivo: {data}",);

                        let data = MeasurementRaw::from_str(data)?;

                        let offset = self.date_offset.unwrap_or(DateOffset {
                            timestamp: data.timestamp,
                            date: Local::now(),
                        });
                        self.date_offset = Some(offset);

                        let data = Measurement::new(offset.timestamp, offset.date, &data);

                        self.tx.send(ThreadMessage::Data(data))?;
                        if let Some(ctx) = self.ctx.as_ref() {
                            ctx.request_repaint();
                        }
                },
                Some(msg) = self.rx.recv() => {
                    match msg {
                        ThreadMessage::StartConnection { address, port, ctx } => {
                            self.ctx = Some(ctx);

                            debug!(
                                "StartConnection recibido por hilo auxiliar con parámetros {address}:{port}",
                            );

                            let remote_addr = if let Ok(remote_ip) = IpAddr::from_str(address.as_str()) {
                                SocketAddr::new(remote_ip, port)
                            } else {
                                tokio::net::lookup_host(format!("{address}:{port}"))
                                    .await?
                                    .next()
                                    .ok_or(anyhow!(
                                        "No se pudo encontrar el dispositivo en {address}:{port}"
                                    ))?
                            };

                            let sock = UdpSocket::bind("0.0.0.0:0").await?;
                            debug!("Socket creado {sock:?}");

                            sock.send_to(b"START", remote_addr).await?;

                            self.connection = Some(Connection::new(remote_addr, sock));
                            self.tx.send(ThreadMessage::ConnectionEstablished)?;
                        },
                        ThreadMessage::Disconnect => {
                            if let Some(connection) = self.connection.as_mut() {
                                let sock = &connection.socket;
                                let remote_addr = connection.remote_addr;
                                sock.send_to(b"END", remote_addr).await?;
                            }

                            self.connection = None;
                            self.date_offset = None;
                        },
                        ThreadMessage::Shutdown => {
                            debug!("Recibida señal de cierre");
                            return Ok(true);
                        }
                        _ => {}
                }
            },
            else => ()
        }

        Ok(false)
    }
}
