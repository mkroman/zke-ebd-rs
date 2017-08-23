//! EBD-USB device communication library and tools.
//! 
//! Communicate via serial to the ZKETECH EBD-USB devices.
//! 
//! Most of the communication is reverse engineered by monitoring the serial communication.
//! 
//! There doesn't seen to be any kind of official documentation for the device.
extern crate serial;
#[macro_use] extern crate quick_error;

/// Errors.
pub mod error;
/// Device handles.
pub mod device;
/// Device-specific device descriptor.
pub mod devices;

