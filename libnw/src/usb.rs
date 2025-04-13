pub use eadk::is_plugged;

/// Interface with the raw `eadk` C api.
///
/// If you don't know what you are doing, use the safe rust implementations.
pub mod eadk {
    unsafe extern "C" {
        /// Checks if the usb is plugged.
        ///
        /// The link process will fail if the calculator is not compatible.
        /// You also may need to increase the EADK_API_LEVEL for this to work.
        #[link_name = "eadk_usb_is_plugged"]
        pub safe fn is_plugged() -> bool;
    }
}
