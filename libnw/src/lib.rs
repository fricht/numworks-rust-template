#![no_std]

/// math-related utilities
pub mod libmath {
    pub use micromath::*;
}

/// hardware interaction
pub mod eadk;

pub mod graphics;
