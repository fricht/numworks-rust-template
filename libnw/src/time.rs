/// Interface with the raw `eadk` C api.\
/// If you don't know what you are doing, use the safe rust implementations.
pub mod raw_api {
    unsafe extern "C" {
        pub fn eadk_timing_msleep(ms: u32);
        pub fn eadk_timing_usleep(us: u32);
        pub fn eadk_timing_millis() -> u64;
    }
}

use raw_api::*;

/// Blocks the thread for a given amount of milliseconds.
pub fn msleep(ms: u32) {
    unsafe { eadk_timing_msleep(ms) }
}

/// Blocks the thread for a given amount of microseconds.
pub fn usleep(us: u32) {
    unsafe { eadk_timing_usleep(us) }
}

/// The number or milliseconds since the start.
///
/// The start of what ? I don't know.\
/// Really, nobody knows.
///
/// But who cares ?\
/// After all, what's the meaning of time ?
///
/// There just can't be a *start*.\
/// Time doesn't live in space.
/// It has no start, no end.
/// Time just flows, as subjective as we can perceive it, as relative as we can measure it.
///
/// > *The clock ticks slowly,*\
/// > *yet years vanish in a breath--*\
/// > *childhood in the wind.*
pub fn monotonic() -> u64 {
    unsafe { eadk_timing_millis() }
}
