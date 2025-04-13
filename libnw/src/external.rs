/// Retrieves the buffer containing the external data needed by this app.
pub fn get_data() -> &'static [u8] {
    unsafe { core::slice::from_raw_parts(eadk::data, eadk::data_size) }
}

/// Interface with the raw `eadk` C api.
///
/// If you don't know what you are doing, use the safe rust implementations.
pub mod eadk {
    unsafe extern "C" {
        /// A pointer to the beginning of the external data slice.
        #[link_name = "eadk_external_data"]
        pub static data: *const u8;

        /// The length of the external data slice.
        #[link_name = "eadk_external_data_size"]
        pub static data_size: usize;
    }
}
