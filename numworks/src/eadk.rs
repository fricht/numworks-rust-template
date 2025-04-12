extern crate alloc;

use crate::shared::*;
use alloc::vec::Vec;

/// Pushes a slice of colors onto the screen.
///
/// It is your responsibility to ensure that the rect and the slice's length match.
///
/// The screen is filled from left to right then top to bottom.
pub fn push_rect(rect: EadkRect, pixels: &[EadkColor]) {
    unsafe {
        eadk_display_push_rect(rect, pixels.as_ptr());
    }
}

/// Draws a rect with the given color.
pub fn push_rect_uniform(rect: EadkRect, color: EadkColor) {
    unsafe {
        eadk_display_push_rect_uniform(rect, color);
    }
}

/// Waits for the screen to finish refreshing.
pub fn wait_for_vblank() {
    unsafe {
        eadk_display_wait_for_vblank();
    }
}

// Is the more than minimal wrapping around the unsafe C functions ?
/// Returns the pixels' color in the given rect.
///
/// The screen is read from left to right then top to bottom.
pub fn pull_rect(rect: EadkRect) -> Vec<EadkColor> {
    let buffer_size = rect.area();
    let mut pixels = Vec::with_capacity(buffer_size as usize);
    unsafe {
        eadk_display_pull_rect(rect, pixels.as_mut_ptr());
        // We need to manually adjust the vec length, otherwise
        // vec.len() will return 0 even though it is fully filled.
        pixels.set_len(buffer_size as usize);
    }
    pixels
}

/// Draws a str to the screen.
///
/// It is your responsibility to end the str with a null byte.
pub fn draw_string(
    text: *const u8,
    x: u16,
    y: u16,
    large_font: bool,
    text_color: EadkColor,
    background_color: EadkColor,
) {
    let point = EadkPoint::new(x, y);
    unsafe {
        eadk_display_draw_string(text, point, large_font, text_color, background_color);
    }
}

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
    fn eadk_display_push_rect(rect: EadkRect, pixels: *const EadkColor);
    fn eadk_display_push_rect_uniform(rect: EadkRect, color: EadkColor);
    fn eadk_display_pull_rect(rect: EadkRect, pixels: *mut EadkColor);
    fn eadk_display_wait_for_vblank() -> bool;
    fn eadk_display_draw_string(
        text: *const u8,
        point: EadkPoint,
        large_font: bool,
        text_color: EadkColor,
        background_color: EadkColor,
    );
    fn eadk_usb_is_plugged() -> bool;
    fn eadk_keyboard_scan() -> u64;
    fn eadk_event_get(timeout: *mut i32) -> EadkKey;
}

/// A point on the screen.
///
/// This is only needed for the eadk_display_draw_string and should not be used elsewhere.
#[repr(C)]
pub struct EadkPoint {
    pub x: u16,
    pub y: u16,
}

impl EadkPoint {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}
