//! Interface with the USB port.
//!
//! This only thing you can do is check if a cable is plugged. So sad.

pub use eadk::is_plugged;

/// Interface with the raw `eadk` C api.
///
/// If you don't know what you are doing, use the safe rust implementations.
pub mod eadk {
    unsafe extern "C" {
        /// Checks if the USB is plugged.
        ///
        /// # Link
        /// This function may fail to link if the calculator is not compatible.
        /// You may also need to increase the `EADK_API_LEVEL` for this to work.
        #[link_name = "eadk_usb_is_plugged"]
        pub safe fn is_plugged() -> bool;
    }
}
