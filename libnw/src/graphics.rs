use micromath::vector::Component;

use crate::eadk::display::*;
use crate::libmath::vector::Vector2d;
use core::array;

/////////////////////////////////////////////////

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
    pub rgb565: u16,
}

/// color utilities
impl Color {
    /// get red color component (5 bits)
    pub fn get_red_raw(&self) -> u8 {
        let r = ((self.rgb565 >> 8) & 0b11111000) as u8;
        r | (r >> 5)
    }

    /// set red color component (5 bits)
    pub fn set_red_raw(&mut self, red: u8) {
        self.rgb565 = (self.rgb565 & 0b0000011111111111) | ((red as u16 & 0b11111000) << 8)
    }

    /// get green color component (6 bits)
    pub fn get_green_raw(&self) -> u8 {
        let g = ((self.rgb565 >> 3) & 0b11111100) as u8;
        g | (g >> 6)
    }

    /// set green color component (6 bits)
    pub fn set_green_raw(&mut self, green: u8) {
        self.rgb565 = (self.rgb565 & 0b1111100000011111) | ((green as u16 & 0b11111100) << 3)
    }

    /// get blue color component (5 bits)
    pub fn get_blue_raw(&self) -> u8 {
        let b = ((self.rgb565 & 0b11111) << 3) as u8;
        b | (b >> 5)
    }

    /// set blue color component (5 bits)
    pub fn set_blue_raw(&mut self, blue: u8) {
        self.rgb565 = (self.rgb565 & 0b1111111111100000) | (blue as u16 >> 3)
    }

    /// linearely interpolate 2 colors
    pub fn lerp(self, target: Self, t: f32) -> Self {
        unimplemented!();
        // let mut c = Color { rgb565: 0 };
        // c.set_red_raw(
        //     (self.get_red_raw() as f32
        //         + (target.get_red_raw() as f32 - self.get_red_raw() as f32) * t) as u16,
        // );
        // c.set_green_raw(
        //     (self.get_green_raw() as f32
        //         + (target.get_green_raw() as f32 - self.get_green_raw() as f32) * t)
        //         as u16,
        // );
        // c.set_blue_raw(
        //     (self.get_blue_raw() as f32
        //         + (target.get_blue_raw() as f32 - self.get_blue_raw() as f32) * t)
        //         as u16,
        // );
        // c
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}
/////////////////////////////////////////

/// Screen buffer to hold data before drawing it
pub struct Buffer {
    data: [Color; 320 * 240],
}

impl Buffer {
    /// creates a new empty buffer (black)
    pub fn new() -> Self {
        Buffer {
            data: array::from_fn(|_| Color { rgb565: 0 }),
        }
    }

    /// draws a rect from the list of colors
    pub fn push_rect(&mut self, rect: Rect, col: &[Color]) {
        for y in rect.y..(rect.y + rect.height) {
            for x in rect.x..(rect.x + rect.width) {
                self.set_at(x, y, col[y as usize * rect.width as usize + x as usize])
            }
        }
    }

    /// draws a rect from the list of colors with the color 'colorkey' not being drawn
    pub fn push_rect_alpha(&mut self, rect: Rect, col: &[Color], colorkey: Color) {
        for y in rect.y..(rect.y + rect.height) {
            for x in rect.x..(rect.x + rect.width) {
                let c = col[y as usize * rect.width as usize + x as usize];
                if c.rgb565 != colorkey.rgb565 {
                    self.set_at(x, y, col[y as usize * rect.width as usize + x as usize])
                }
            }
        }
    }

    /// draws a unicolor rect
    pub fn push_rect_uniform(&mut self, rect: Rect, col: Color) {
        for x in rect.x..(rect.x + rect.width) {
            for y in rect.y..(rect.y + rect.height) {
                self.set_at(x, y, col)
            }
        }
    }

    /// clears the buffer with the given color
    pub fn clear(&mut self, c: Color) {
        self.push_rect_uniform(
            Rect {
                x: 0,
                y: 0,
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
            },
            c,
        )
    }

    /// set buffer pixel
    pub fn set_at(&mut self, x: u16, y: u16, c: Color) {
        if x < SCREEN_WIDTH && y < SCREEN_HEIGHT {
            self.data[320 * y as usize + x as usize] = c
        }
    }

    /// draws the buffer to the screen
    pub fn render(&self) {
        wait_for_vblank();
        push_rect(
            Rect {
                x: 0,
                y: 0,
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
            },
            &self.data,
        )
    }

    /// draws a line
    pub fn line(&mut self, start: Vector2d, end: Vector2d, c: Color) {
        let min_x = f32::min(start.x, end.x);
        let max_x = f32::max(start.x, end.x);
        let min_y = f32::min(start.y, end.y);
        let max_y = f32::max(start.y, end.y);
        let x_count = (max_x - min_x) as usize;
        let y_count = (max_y - min_y) as usize;
        let count;
        let dx;
        let dy;
        if x_count > y_count {
            count = x_count as usize;
            dx = 1;
            dy = 0;
        } else {
            count = y_count as usize;
            dx = 0;
            dy = 1;
        }
        let delta = 1. / (count as f32);
        for incrementer in 0..count {
            let v = start.lerp(end, delta / 2. + delta * incrementer as f32);
            if v.x < 0. || v.y < 0. {
                continue;
            }
            let x = v.x as u16;
            let y = v.y as u16;
            self.set_at(x, y, c);
            self.set_at(x + dx, y + dy, c);
        }
    }

    /// draws a circle
    pub fn circle<T: Component>(&mut self, center: Vector2d<T>, radius: T, col: Color) {
        let sqd = radius * radius;
        for x in (center.x - radius)..((center.x + radius) + 1) {
            for y in ((center.y - radius) as usize)..((center.y + radius) as usize + 1) {
                let dx = x as f32 - center.x;
                let dy = y as f32 - center.y;
                if dx * dx + dy * dy <= sqd {
                    self.set_at(x as u16, y as u16, col);
                }
            }
        }
    }
}
