extern crate alloc;

use crate::shared::*;

/// Checks if the usb is plugged.
///
/// The link process will fail if the calculator is not compatible.
/// You also may need to increase the EADK_API_LEVEL for this to work.
pub fn usb_is_plugged() -> bool {
    unsafe { eadk_usb_is_plugged() }
}

/// Retrieves the current state of the keyboard.
pub fn keyboard_scan() -> EadkKeyboardState {
    unsafe { EadkKeyboardState(eadk_keyboard_scan()) }
}

/// Checks if the key was pressed in the given state.
pub fn keyboard_key_down(keyboard_state: EadkKeyboardState, key: EadkRawKey) -> bool {
    (keyboard_state.0 >> (key as u8)) & 1 != 0
}

/// Waits until a key (or combination of keys) is pressed,
/// or until `timeout` expires.
///
/// Only detects new key presses. Holding a key will not
/// trigger multiple events if this function is called
/// repeatedly while the key remains pressed.
pub fn event_get(timeout: i32) -> EadkKey {
    // copy the value
    let mut timeout = timeout;
    unsafe { eadk_event_get(&mut timeout as *mut _) }
}

unsafe extern "C" {
    fn eadk_usb_is_plugged() -> bool;
    fn eadk_keyboard_scan() -> u64;
    fn eadk_event_get(timeout: *mut i32) -> EadkKey;
}
