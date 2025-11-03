use std::rc::Rc;

use anyhow::Result;
use serialport::{SerialPort, SerialPortInfo};

const MAGIC_HEADER: [u8; 5] = [0x75, 0xE0, 0x5E, 0xB1, 0xC0];

pub fn attempt_handshake(mut port: Box<dyn SerialPort>) -> Result<Box<dyn SerialPort>> {
    port.write(&MAGIC_HEADER);
    Ok(port)
}

pub fn get_serial_ports() -> Vec<Rc<SerialPortInfo>> {
    serialport::available_ports()
        .expect("Ocurrió un error al obtener la lista de puertos disponibles")
        .into_iter()
        .map(Rc::new)
        .collect()
}
