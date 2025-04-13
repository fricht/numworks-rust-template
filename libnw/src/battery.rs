pub use eadk::*;

/// Interface with the raw `eadk` C api.
///
/// If you don't know what you are doing, use the safe rust implementations.
pub mod eadk {
    unsafe extern "C" {
        /// Checks if the battery is in charge.
        ///
        /// The link process will fail if the calculator is not compatible.
        /// You may also need to increase the `EADK_API_LEVEL` for this to work.
        #[link_name = "eadk_battery_is_charging"]
        pub safe fn is_charging() -> bool;

        /// Returns the battery level.
        ///
        /// The link process will fail if the calculator is not compatible.
        /// You may also need to increase the `EADK_API_LEVEL` for this to work.
        #[link_name = "eadk_battery_level"]
        pub safe fn get_level() -> u8;

        /// Returns the battery voltage.
        ///
        /// The link process will fail if the calculator is not compatible.
        /// You may also need to increase the `EADK_API_LEVEL` for this to work.
        #[link_name = "eadk_battery_voltage"]
        pub safe fn get_voltage() -> f32;
    }
}
