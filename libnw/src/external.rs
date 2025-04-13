/// Interface with the raw `eadk` C api.\
/// If you don't know what you are doing, use the safe rust implementations.
pub mod raw_api {
    unsafe extern "C" {
        pub static eadk_external_data: *const u8;
        pub static eadk_external_data_size: usize;
    }
}

use raw_api::*;

/// Retrieves the buffer containing the external data needed by this app.
pub fn get_data() -> &'static [u8] {
    unsafe { core::slice::from_raw_parts(eadk_external_data, eadk_external_data_size) }
}
