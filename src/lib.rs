extern crate serial;
#[macro_use] extern crate quick_error;

pub mod error;
pub mod device;
pub mod devices;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
