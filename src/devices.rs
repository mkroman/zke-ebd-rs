#[derive(Debug, Clone)]
pub struct DeviceDescriptor {
  pub name: &'static str,
  pub current_divider: u16,
  pub max_current: u32,
  pub max_power: u32,
}

#[allow(dead_code)]
pub const EBD_USB_V1: DeviceDescriptor = DeviceDescriptor {
  name: "EBD-USB V1",
  current_divider: 1000,
  max_current: 3000,
  max_power: 24_000,
};

#[allow(dead_code)]
pub static EBD_USB_V2: DeviceDescriptor = DeviceDescriptor {
  name: "EBD-USB V2",
  current_divider: 1000,
  max_current: 3000,
  max_power: 24_000,
};

#[allow(dead_code)]
pub const EBD_USB_PLUS: DeviceDescriptor = DeviceDescriptor {
  name: "EBD-USB Plus",
  current_divider: 10_000,
  max_current: 5000,
  max_power: 50_000,
};

#[allow(dead_code)]
pub static EBD_USB_PLUS_PLUS: DeviceDescriptor = DeviceDescriptor {
  name: "EBD-USB Plus+",
  current_divider: 5000,
  max_current: 5000,
  max_power: 50_000,
};