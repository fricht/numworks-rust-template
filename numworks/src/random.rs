use crate::display::{Color, SCREEN_HEIGHT, SCREEN_WIDTH, ScreenRect};

unsafe extern "C" {
    fn eadk_random() -> u32;
}

/// Returns a random u32.
pub fn random() -> u32 {
    unsafe { eadk_random() }
}

/// Returns a random unsigned integer in [a, b[.
pub fn randuint(a: u32, b: u32) -> u32 {
    let range = b - a;
    let random_number = random() % range;
    random_number + a
}

/// Returns a random color
pub fn random_color() -> Color {
    Color::from_rgb(random() as u8, random() as u8, random() as u8)
}

/// Returns a random rect.
/// The rect is fully contained in screen.
pub fn random_rect() -> ScreenRect {
    let x = randuint(0, SCREEN_WIDTH as u32) as u16;
    let y = randuint(0, SCREEN_HEIGHT as u32) as u16;
    let width = randuint(0, (SCREEN_WIDTH - x) as u32) as u16;
    let height = randuint(0, (SCREEN_HEIGHT - y) as u32) as u16;
    ScreenRect::new(x, y, width, height)
}
