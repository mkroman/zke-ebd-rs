#[macro_use]
extern crate clap;
extern crate zke_ebd;

use clap::{App, Arg};
use zke_ebd::device::Device;

#[cfg(target_os = "windows")]
const DEFAULT_DEVICE: &'static str = "COM1";
#[cfg(target_os = "linux")]
const DEFAULT_DEVICE: &'static str = "/dev/ttyUSB0";

fn try_main() -> Result<(), zke_ebd::error::Error> {
  let matches = App::new("ZKE EBD-USB")
                  .version(crate_version!())
                  .author(crate_authors!())
                  .about("Monitoring tool for the ZKE EBD-USB devices")
                  .arg(Arg::with_name("device")
                    .short("d")
                    .long("device")
                    .value_name("DEVICE")
                    .help("Sets serial device")
                    .required(true)
                    .takes_value(true)
                    .default_value(DEFAULT_DEVICE))
                  .get_matches();
  let device_arg = matches.value_of_os("device").unwrap();
  let mut device: Device<zke_ebd::devices::EbdUsbPlus> = Device::open(device_arg)?;

  device.monitor(|measurement| {
    println!("{:?}", measurement);
  })?;

  Ok(())
}

fn main() {
  match try_main() {
    Ok(()) => {}
    Err(e) => println!("Error: {:?}", e)
  }
}