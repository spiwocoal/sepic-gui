use std::{rc::Rc, str::from_utf8};

use anyhow::{Result, anyhow};
use log::debug;
use serialport::{SerialPort, SerialPortInfo};

const ACK: [u8; 1] = [0x06];
const NACK: [u8; 1] = [0x15];

pub fn get_serial_ports() -> Vec<Rc<SerialPortInfo>> {
    serialport::available_ports()
        .expect("Ocurrió un error al obtener la lista de puertos disponibles")
        .into_iter()
        .map(Rc::new)
        .collect()
}

pub fn attempt_handshake(mut port: Box<dyn SerialPort>) -> Result<Box<dyn SerialPort>> {
    let mut buf = [0u8; 64];

    port.write_all(&[0x02, 0x05, 0x03])?;
    let _ = port.read(&mut buf)?;
    let buf = buf
        .get(0..ACK.len())
        .ok_or(anyhow!("No se pudo seccionar la respuesta del dispositivo"))?;
    if buf.iter().zip(ACK.iter()).all(|(a, b)| a == b) {
        return Ok(port);
    }

    Err(anyhow!(
        "El dispositivo no responde correctamente, {:x?}",
        buf
    ))
}

pub fn set_duty(port: &mut Box<dyn SerialPort>, duty_cycle: f32) -> Result<()> {
    let mut buf = [0u8; 64];
    let query = format!("\x02DCS {:x}\x03", (duty_cycle * ((1 << 9) as f32)) as u32);
    debug!("Enviando mensaje {:?}", query);

    port.write_all(query.as_bytes())?;

    let _ = port.read(&mut buf)?;
    debug!("Mensaje recibido: {:?}", from_utf8(&buf));
    let buf = buf
        .get(0..ACK.len())
        .ok_or(anyhow!("No se pudo seccionar la respuesta del dispositivo"))?;
    if buf.iter().zip(ACK.iter()).all(|(a, b)| a == b) {
        return Ok(());
    }

    Err(anyhow!(
        "El dispositivo no responde correctamente, {:x?}",
        buf
    ))
}

pub fn ramp_duty(
    port: &mut Box<dyn SerialPort>,
    duty_start: f32,
    duty_end: f32,
    tspan: u32,
) -> Result<()> {
    let mut buf = [0u8; 64];
    let query = format!(
        "\x02DCR {:#x} {:#x} {:#x}\x03",
        (duty_start * ((1 << 9) as f32)) as u32,
        (duty_end * ((1 << 9) as f32)) as u32,
        tspan
    );
    debug!("Enviando mensaje {:?}", query);

    port.write_all(query.as_bytes())?;

    let _ = port.read(&mut buf)?;
    debug!("Mensaje recibido: {:?}", buf);
    let buf = buf
        .get(0..ACK.len())
        .ok_or(anyhow!("No se pudo seccionar la respuesta del dispositivo"))?;
    if buf.iter().zip(ACK.iter()).all(|(a, b)| a == b) {
        return Ok(());
    }

    Err(anyhow!(
        "El dispositivo no responde correctamente, {:x?}",
        buf
    ))
}

pub fn set_frequency(port: &mut Box<dyn SerialPort>, frequency: f32) -> Result<()> {
    let mut buf = [0u8; 64];
    let query = format!("\x02FQS {:#x}\x03", (frequency) as u32);
    debug!("Enviando mensaje {:?}", query);

    port.write_all(query.as_bytes())?;

    let _ = port.read(&mut buf)?;
    debug!("Mensaje recibido: {:?}", buf);
    let buf = buf
        .get(0..ACK.len())
        .ok_or(anyhow!("No se pudo seccionar la respuesta del dispositivo"))?;
    if buf.iter().zip(ACK.iter()).all(|(a, b)| a == b) {
        return Ok(());
    }

    Err(anyhow!(
        "El dispositivo no responde correctamente, {:x?}",
        buf
    ))
}
