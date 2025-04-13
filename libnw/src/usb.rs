/// Interface with the raw `eadk` C api.\
/// If you don't know what you are doing, use the safe rust implementations.
pub mod raw_api {
    unsafe extern "C" {
        pub fn eadk_usb_is_plugged() -> bool;
    }
}

use raw_api::*;

/// Checks if the usb is plugged.
///
/// The link process will fail if the calculator is not compatible.
/// You also may need to increase the EADK_API_LEVEL for this to work.
pub fn is_usb_plugged() -> bool {
    unsafe { eadk_usb_is_plugged() }
}
