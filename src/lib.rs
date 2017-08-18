extern crate serial;
#[macro_use] extern crate quick_error;

pub mod error;
pub mod device;
pub mod devices;

pub trait EbdDevice {
  #[inline(always)]
  fn current_divider() -> u16;

  #[inline(always)]
  fn max_current() -> u32;

  #[inline(always)]
  fn max_power() -> u32;

  #[inline(always)]
  fn device_name() -> &'static str;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
