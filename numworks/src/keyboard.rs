use crate::eadk;

pub use crate::shared::{EadkKey as Key, EadkRawKey as RawKey};

/// Waits for `timeout` or until a key is pressed.
///
/// If the timeout is reached, returns `None`.
pub fn wait_for_input(timeout_ms: i32) -> Option<Key> {
    match eadk::event_get(timeout_ms) {
        Key::None => None,
        key => Some(key),
    }
}

/// Checks if the given key is pressed.
pub fn is_pressed(key: RawKey) -> bool {
    let keyboard_state = eadk::keyboard_scan();
    eadk::keyboard_key_down(keyboard_state, key)
}

pub fn currently_pressed() -> Key {
    eadk::event_get(1)
}

pub use eadk::keyboard_scan as state;