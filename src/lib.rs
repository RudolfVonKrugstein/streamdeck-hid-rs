// Publicly expose the modules
mod device;
mod error;
mod hid_api_traits;
mod image;
mod type_info;

pub use device::*;
pub use error::*;
pub use type_info::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
