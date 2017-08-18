use super::EbdDevice;

// EBD-USB V1
pub struct EbdUsbV1;

impl EbdDevice for EbdUsbV1 {
  fn current_divider() -> u16 {
    1000
  }

  fn max_current() -> u32 {
    3000
  }

  fn max_power() -> u32 {
    24000
  }

  fn device_name() -> &'static str {
    "EDB-USB V1"
  }
}

// EBD-USB V2
pub struct EbdUsbV2;

impl EbdDevice for EbdUsbV2 {
  fn current_divider() -> u16 {
    1000
  }

  fn max_current() -> u32 {
    3000
  }

  fn max_power() -> u32 {
    24000
  }

  fn device_name() -> &'static str {
    "EDB-USB V2"
  }
}

// EBD-USB Plus
pub struct EbdUsbPlus;

impl EbdDevice for EbdUsbPlus {
  fn current_divider() -> u16 {
    10000
  }

  fn max_current() -> u32 {
    5000
  }

  fn max_power() -> u32 {
    50000
  }

  fn device_name() -> &'static str {
    "EDB-USB Plus"
  }
}

// EBD-USB Plus+
pub struct EbdUsbPlusPlus;

impl EbdDevice for EbdUsbPlusPlus {
  fn current_divider() -> u16 {
    5000
  }

  fn max_current() -> u32 {
    5000
  }

  fn max_power() -> u32 {
    50000
  }

  fn device_name() -> &'static str {
    "EDB-USB Plus+"
  }
}