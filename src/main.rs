#![no_std]
#![no_main]

use libnw::libmath;

pub mod eadk;
pub mod graphics;
pub mod math;

use eadk::Color;
use graphics::Buffer;

#[used]
#[link_section = ".rodata.eadk_app_name"]
pub static EADK_APP_NAME: [u8; 8] = *b"Numrust\0";

#[used]
#[link_section = ".rodata.eadk_api_level"]
pub static EADK_APP_API_LEVEL: u32 = 0;

#[used]
#[link_section = ".rodata.eadk_app_icon"]
pub static EADK_APP_ICON: [u8; 6186] = *include_bytes!("../target/icon.nwi");

#[no_mangle]
pub fn main() {
    let mut buff = Buffer::new();
    buff.clear(Color { rgb565: 0x001F });
    buff.render();
    // empty main loop
    loop {}
}
