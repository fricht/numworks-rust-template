//! Interface to external data embedded with the application when sending it to the calculator.
//!
//! This module provides a safe way to access a static byte slice passed
//! from the C side (EADK). This is useful for bundling assets such as
//! images, fonts or any binary data required by the app.

/// Returns a static slice containing external binary data bundled with the application.
///
pub fn get_data() -> &'static [u8] {
    unsafe {
        // SAFETY: The underlying pointer is provided by EADK. It is assumed to be
        // non-null and valid for the entire duration of the application.
        debug_assert!(!eadk::data.is_null());
        core::slice::from_raw_parts(eadk::data, eadk::data_size)
    }
}

/// Interface with the raw `eadk` C api.
///
/// If you don't know what you are doing, use the safe rust implementations.
pub mod eadk {
    unsafe extern "C" {
        /// A pointer to the beginning of the external data slice.
        /// 
        /// # Safety
        /// This pointer should always be used with `eadk::data_size`.
        #[link_name = "eadk_external_data"]
        pub static data: *const u8;

        /// The length of the external data slice.
        #[link_name = "eadk_external_data_size"]
        pub safe static data_size: usize;
    }
}
