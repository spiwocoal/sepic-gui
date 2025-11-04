use std::rc::Rc;

use anyhow::{Result, anyhow};
use serialport::{SerialPort, SerialPortInfo};

const MAGIC_HEADER: [u8; 6] = [0x75, 0xE0, 0x5E, 0xB1, 0xC0, 0x00];

pub fn attempt_handshake(mut port: Box<dyn SerialPort>) -> Result<Box<dyn SerialPort>> {
    let mut buf = [0u8; 512];

    port.write_all(b"hello\n")?;
    let bytes_recvd = port.read(&mut buf)?;
    if bytes_recvd == MAGIC_HEADER.len() {
        let buf = buf
            .get(0..MAGIC_HEADER.len())
            .ok_or(anyhow!("No se pudo seccionar la respuesta del dispositivo"))?;
        if buf.iter().zip(MAGIC_HEADER.iter()).all(|(a, b)| a == b) {
            return Ok(port);
        }
    }

    Err(anyhow!("El dispositivo no responde correctamente"))
}

pub fn get_serial_ports() -> Vec<Rc<SerialPortInfo>> {
    serialport::available_ports()
        .expect("Ocurrió un error al obtener la lista de puertos disponibles")
        .into_iter()
        .map(Rc::new)
        .collect()
}
