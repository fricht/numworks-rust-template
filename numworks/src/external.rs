unsafe extern "C" {
    static eadk_external_data: *const u8;
    static eadk_external_data_size: usize;
}

/// Retrieves the buffer containing the external data needed by this app.
pub fn external_data() -> &'static [u8] {
    unsafe { core::slice::from_raw_parts(eadk_external_data, eadk_external_data_size) }
}
