// Publicly expose the modules
mod type_info;
mod device;
mod error;
mod image;
mod hid_api_traits;

pub use type_info::*;
pub use device::*;
pub use error::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
