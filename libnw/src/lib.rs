#![no_std]

#[used]
#[unsafe(link_section = ".rodata.eadk_api_level")]
static EADK_API_LEVEL: u32 = 0;

mod heap;

pub mod backlight;
pub mod battery;
pub mod display;
pub mod external;
pub mod keyboard;
pub mod random;
pub mod time;
pub mod usb;
