use std::io::{Write, Read};
use std::ffi::OsStr;
use serial::{SerialPort, BaudRate};
use super::error::Error;
use super::devices::DeviceDescriptor;

const INIT_SEQUENCE: [u8; 10] = [0xfa, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0xf8];
const EBD_START_BYTE: u8 = 0xfa;
const EBD_STOP_BYTE: u8 = 0xf8;

/// Device handle.
pub struct Device {
  port: ::serial::SystemPort,
  device: DeviceDescriptor,
}

/// Generates a checksum byte for the passed `data`.
fn generate_checksum(data: &[u8]) -> u8 {
  data.iter().fold(0u8, |acc, b| acc ^ b)
}

/// Decodes the bytes passed as `buf` as a current, divided by the device-specific `current_divider`.
fn decode_current(buf: &[u8], current_divider: u16) -> f64 {
  let b1 = buf[0] as i32;
  let b2 = buf[1] as i32;

  (((b1 * 256 + b2) - (b1 * 256 + b2) / 256 * 16) as f64 / current_divider as f64) as f64
}

/// Decodes the bytes passed as `buf` as a voltage.
fn decode_voltage(buf: &[u8]) -> f64 {
  let b1 = buf[0] as i32;
  let b2 = buf[1] as i32;

  let voltage: f64 = (((b1 * 256 + b2 - 320) -
    (b1 * 256 + b2 - 5120) / 256 * 16) as f64 / 1000.) as f64;

  voltage
}

/// Measurement values.
#[derive(Debug)]
pub struct Measurement {
  /// The voltage in mV.
  pub voltage: f64,
  /// The current in mAh.
  pub current: f64,
  /// The checksum byte.
  pub checksum: u8,
}

impl Measurement {
  /// Parses the given `buf` as a measurement packet.
  /// 
  /// # Arguments
  /// * `current_divider` - device-specific current divider value.
  pub fn parse(buf: &[u8; 19], current_divider: u16) -> Result<Measurement, Error> {
    let checksum = generate_checksum(&buf[1..17]);

    if buf[17] != checksum {
      return Err(Error::InvalidChecksum);
    }

    let voltage = decode_voltage(&buf[4..6]);
    let current = decode_current(&buf[2..4], current_divider);

    Ok(Measurement {
      voltage: voltage,
      current: current,
      checksum: buf[17],
    })
  }
}

impl Device {
  /// Opens a serial connection device on the specified `port` and initiates monitoring.
  pub fn open<T: AsRef<OsStr> + ?Sized>(port: &T, device: DeviceDescriptor) -> Result<Device, Error> {
    let mut serial_port = ::serial::open(port)?;

    serial_port.reconfigure(&|ref mut settings| {
      settings.set_baud_rate(BaudRate::Baud9600)?;

      Ok(())
    })?;

    serial_port.write(&INIT_SEQUENCE)?;
    serial_port.set_timeout(::std::time::Duration::from_millis(2500))?;

    Ok(Device {
      port: serial_port,
      device: device,
    })
  }

  /// Starts monitoring the device, calling `func` with each successful measurement read from the
  /// device. The interval between each measurement depends on the device.
  /// 
  /// This is a blocking operation.
  /// 
  /// # Arguments
  /// 
  /// * `func` - the callback function to handle measurements.
  /// 
  /// # Return value
  /// 
  /// When the device sends an illegaly structured packet, the function returns
  /// `Error::InvalidStartByte` or `Error::InvalidStopByte`
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

      match Measurement::parse(&buf, self.device.current_divider) {
        Ok(measurement) => func(measurement),
        Err(e) => {
          println!("Error when reading measurement: {} - buf: {:?}", e, &buf);
        }
      }
    }
  }
}
