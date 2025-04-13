#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use myapp::{self, ExitBehaviour};

/// Defines the name of the application.
#[used]
#[unsafe(link_section = ".rodata.eadk_app_name")]
static EADK_APP_NAME: [u8; 7] = *b"My app\0";

/// Defines the icon of the application.
#[used]
#[unsafe(link_section = ".rodata.eadk_app_icon")]
static EADK_APP_ICON: [u8; 6186] = *include_bytes!("../target/icon.nwi");

/// The entry point of the application.
///
/// The should only be setup code, the behavior of the application
/// should be defined in the main function of lib.rs.
#[unsafe(no_mangle)]
fn main() {
    // THIS MUST BE THE FIRST THING
    // Or at least, the heap must have been initialized before
    // attempting to use any heap-allocated struct.
    libnw::init_heap!(1024);

    // Run the core of the app.
    let mut behaviour = ExitBehaviour::Restart;
    while let ExitBehaviour::Restart = behaviour {
        behaviour = myapp::main();
    }
    if let ExitBehaviour::Hang = behaviour {
        loop {}
    }
}

/// Handlers for panic and allocation error
mod no_std {
    extern crate alloc;

    use alloc::format;
    use libnw::display::{self, CHAR_HEIGHT, CHAR_WIDTH, Color, SCREEN_HEIGHT, SCREEN_WIDTH};

    /// This function is called when the application panics.
    #[panic_handler]
    fn panic(panic: &core::panic::PanicInfo<'_>) -> ! {
        display::clear(Color::RED);
        let error_msg = panic.message().as_str().unwrap_or("No panic message");
        render_error(error_msg);
        loop {}
    }

    /// This function is called when an allocation error occur.
    #[alloc_error_handler]
    fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
        display::clear(Color::RED);
        let size_needed = layout.size();
        let error_msg = format!("Allocation error, {size_needed} bytes needed");
        render_error(&error_msg);
        loop {}
    }

    fn render_error(message: &str) {
        display::draw_string(
            message,
            SCREEN_WIDTH / 2 - message.len() as u16 * CHAR_WIDTH / 2,
            SCREEN_HEIGHT / 2 - CHAR_HEIGHT / 2,
            false,
            Color::RED,
            Color::WHITE,
        );
    }
}
