use std::io;
use serial;

quick_error! {
  #[derive(Debug)]
  pub enum Error {
    InvalidChecksum {
      from()
      description("invalid checksum")
    }
    InvalidStartByte
    InvalidStopByte
    Serial(err: serial::Error) {
      from()
      description("serial error")
      cause(err)
    }
    Io(err: io::Error) {
      from()
      description("IO error")
      cause(err)
    }
  }
}