use crate::{
    display::{Color, SCREEN_HEIGHT, SCREEN_WIDTH, ScreenRect},
    eadk,
};

pub fn randuint(a: u32, b: u32) -> u32 {
    let range = b - a;
    let random_number = eadk::random() % range;
    random_number + a
}

pub fn random_color() -> Color {
    Color::from_rgb(
        randuint(0, u8::MAX as u32) as u8,
        randuint(0, u8::MAX as u32) as u8,
        randuint(0, u8::MAX as u32) as u8,
    )
}

pub fn random_rect() -> ScreenRect {
    let x = randuint(0, SCREEN_WIDTH as u32) as u16;
    let y = randuint(0, SCREEN_HEIGHT as u32) as u16;
    let width = randuint(0, SCREEN_WIDTH as u32) as u16;
    let height = randuint(0, SCREEN_HEIGHT as u32) as u16;
    ScreenRect::new(x, y, width, height)
}
