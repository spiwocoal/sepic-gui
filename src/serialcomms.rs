use std::{
    io::{Read, Write},
    rc::Rc,
};

use anyhow::{Result, anyhow};
use log::debug;
use serialport::{SerialPort, SerialPortInfo};

const STX: [u8; 1] = [0x02];
const ETX: [u8; 1] = [0x03];
const ENQ: [u8; 1] = [0x05];

const ACK: [u8; 1] = [0x06];
const NACK: [u8; 1] = [0x15];

#[expect(unsafe_code)]
fn from_utf8_escaped(body: &[u8]) -> String {
    unsafe {
        str::from_utf8_unchecked(body)
            .trim_end_matches('\0')
            .escape_default()
            .to_string()
    }
}

pub fn get_serial_ports() -> Vec<Rc<SerialPortInfo>> {
    serialport::available_ports()
        .expect("Ocurrió un error al obtener la lista de puertos disponibles")
        .into_iter()
        .map(Rc::new)
        .collect()
}

pub fn send_command<'a, T: Write + Read + ?Sized>(port: &'a mut Box<T>, cmd: &[u8]) -> Result<()> {
    let mut input_buf = [0u8; 64];
    let output_buf: Vec<u8> = vec![&STX, cmd, &ETX].concat();
    let output_buf = output_buf.as_slice();

    debug!("Enviando mensaje `{}`", from_utf8_escaped(output_buf));

    port.write_all(output_buf)?;
    let _ = port.read(&mut input_buf)?;

    debug!("Mensaje recibido `{}`", from_utf8_escaped(&input_buf));

    input_buf
        .get(0..ACK.len())
        .ok_or(anyhow!("No se pudo seccionar la respuesta del dispositivo"))?
        .iter()
        .zip(ACK.iter())
        .all(|(a, b)| a == b)
        .then_some(())
        .ok_or(anyhow!(
            "La respuesta del dispositivo es inválida `{}`",
            from_utf8_escaped(&input_buf)
        ))
}

fn send_command_owned<T: Write + Read + ?Sized>(mut port: Box<T>, cmd: &[u8]) -> Result<Box<T>> {
    match send_command(&mut port, cmd) {
        Ok(_) => Ok(port),
        Err(e) => Err(e),
    }
}

pub fn attempt_handshake<T: Write + Read + ?Sized>(port: Box<T>) -> Result<Box<T>> {
    send_command_owned(port, &ENQ)
}

pub fn set_duty<T: Write + Read + ?Sized>(port: &mut Box<T>, duty_cycle: f32) -> Result<()> {
    send_command(
        port,
        format!("DCS {:#x}", (duty_cycle * ((1 << 9) as f32)) as u32).as_bytes(),
    )
}

pub fn ramp_duty(
    port: &mut Box<dyn SerialPort>,
    duty_start: f32,
    duty_end: f32,
    tspan: u32,
) -> Result<()> {
    send_command(
        port,
        format!(
            "DCR {:#x} {:#x} {:#x}",
            (duty_start * ((1 << 9) as f32)) as u32,
            (duty_end * ((1 << 9) as f32)) as u32,
            tspan
        )
        .as_bytes(),
    )
}

pub fn set_frequency(port: &mut Box<dyn SerialPort>, frequency: f32) -> Result<()> {
    send_command(port, format!("FQS {:#x}", frequency as u32).as_bytes())
}
