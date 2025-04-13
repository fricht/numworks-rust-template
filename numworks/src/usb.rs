/// Checks if the usb is plugged.
///
/// The link process will fail if the calculator is not compatible.
/// You also may need to increase the EADK_API_LEVEL for this to work.
pub fn usb_is_plugged() -> bool {
    unsafe { eadk_usb_is_plugged() }
}

unsafe extern "C" {
    fn eadk_usb_is_plugged() -> bool;
}
