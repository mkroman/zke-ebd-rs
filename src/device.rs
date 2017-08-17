use std::io::{Write, Read};
use std::ffi::OsStr;
use serial::{SerialPort, BaudRate};
use super::error::Error;
use std::marker::PhantomData;

pub const INIT_SEQUENCE: [u8; 10] = [0xfa, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0xf8];
const EBD_START_BYTE: u8 = 0xfa;
const EBD_STOP_BYTE: u8 = 0xf8;

pub struct Device<D> {
  port: ::serial::SystemPort,
  phantom: PhantomData<D>
}

pub trait EbdDevice {
  const CURRENT_DIVIDER: u16;
  const MAX_CURRENT: u32;
  const MAX_POWER: u32;
  const DEVICE_NAME: &'static str;
  const PACKET_SIZE: usize;
}

// EBD-USB V1
pub struct EbdUsbV1;

impl EbdDevice for EbdUsbV1 {
  const CURRENT_DIVIDER: u16 = 1000;
  const MAX_CURRENT: u32 = 3000;
  const MAX_POWER: u32 = 24000;
  const DEVICE_NAME: &'static str = "EBD-USB V1";
  const PACKET_SIZE: usize = 19;
}

// EBD-USB V2
pub struct EbdUsbV2;

impl EbdDevice for EbdUsbV2 {
  const CURRENT_DIVIDER: u16 = 1000;
  const MAX_CURRENT: u32 = 3000;
  const MAX_POWER: u32 = 24000;
  const DEVICE_NAME: &'static str = "EBD-USB V2";
  const PACKET_SIZE: usize = 19;
}

// EBD-USB Plus
pub struct EbdUsbPlus;

impl EbdDevice for EbdUsbPlus {
  const CURRENT_DIVIDER: u16 = 10000;
  const MAX_CURRENT: u32 = 5000;
  const MAX_POWER: u32 = 50000;
  const DEVICE_NAME: &'static str = "EBD-USB Plus";
  const PACKET_SIZE: usize = 19;
}

// EBD-USB Plus+
pub struct EbdUsbPlusPlus;

impl EbdDevice for EbdUsbPlusPlus {
  const CURRENT_DIVIDER: u16 = 5000;
  const MAX_CURRENT: u32 = 5000;
  const MAX_POWER: u32 = 50000;
  const DEVICE_NAME: &'static str = "EBD-USB Plus+";
  const PACKET_SIZE: usize = 19;
}

fn generate_checksum(data: &[u8]) -> u8 {
  data.iter().fold(0u8, |acc, b| acc ^ b)
}


fn decode_current(buf: &[u8], current_divider: u16) -> f64 {
  let b1 = buf[0] as i32;
  let b2 = buf[1] as i32;

  (((b1 * 256 + b2) - (b1 * 256 + b2) / 256 * 16) as f64 / current_divider as f64) as f64
}

fn decode_voltage(buf: &[u8]) -> f64 {
  let b1 = buf[0] as i32;
  let b2 = buf[1] as i32;

  let voltage: f64 = (((b1 * 256 + b2 - 320) -
    (b1 * 256 + b2 - 5120) / 256 * 16) as f64 / 1000.) as f64;

  voltage
}

#[derive(Debug)]
pub struct Measurement {
  pub voltage: f64,
  pub current: f64,
}

impl Measurement {
  pub fn parse(buf: &[u8; 19], current_divider: u16) -> Result<Measurement, Error> {
    let checksum = generate_checksum(&buf[1..17]);

    if buf[17] != checksum {
      return Err(Error::InvalidChecksum);
    }

    let voltage = decode_voltage(&buf[4..6]);
    let current = decode_current(&buf[2..4], current_divider);

    Ok(Measurement {
      voltage: voltage,
      current: current
    })
  }
}

impl<D: EbdDevice> Device<D> {
  pub fn open<T: AsRef<OsStr> + ?Sized>(port: &T) -> Result<Device<D>, Error> {
    let mut serial_port = ::serial::open(port)?;

    serial_port.reconfigure(&|ref mut settings| {
      settings.set_baud_rate(BaudRate::Baud9600)?;

      Ok(())
    })?;

    serial_port.write(&INIT_SEQUENCE)?;
    serial_port.set_timeout(::std::time::Duration::from_millis(2500))?;

    Ok(Device {
      port: serial_port,
      phantom: PhantomData
    })
  }

  pub fn monitor<F: Fn(Measurement)>(&mut self, func: F) -> Result<(), Error> {
    let mut buf: [u8; 19] = [0; 19];

    loop {
      self.port.read_exact(&mut buf)?;

      if buf[0] != EBD_START_BYTE {
        return Err(Error::InvalidStartByte);
      }

      if buf[18] != EBD_STOP_BYTE {
        return Err(Error::InvalidStopByte);
      }

      match Measurement::parse(&buf, D::CURRENT_DIVIDER) {
        Ok(measurement) => func(measurement),
        Err(e) => {
          println!("Error when reading measurement: {} - buf: {:?}", e, &buf);
        }
      }
    }
  }

}