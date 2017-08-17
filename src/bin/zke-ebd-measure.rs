extern crate zke_ebd;

use zke_ebd::device::Device;

fn try_main() -> Result<(), zke_ebd::error::Error> {
  let mut device = Device::open::<_,zke_ebd::device::EbdUsbPlus>("COM7")?;

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