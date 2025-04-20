//! Day 13, you are now able to control the time.

pub use eadk::*;

/// Interface with the raw `eadk` C api.
///
/// If you don't know what you are doing, use the safe rust implementations.
pub mod eadk {
    unsafe extern "C" {
        /// Blocks the thread for a given amount of milliseconds.
        #[link_name = "eadk_timing_msleep"]
        pub safe fn msleep(ms: u32);

        /// Blocks the thread for a given amount of microseconds.
        #[link_name = "eadk_timing_usleep"]
        pub safe fn usleep(us: u32);

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
        #[link_name = "eadk_timing_millis"]
        pub safe fn monotonic() -> u64;
    }
}
