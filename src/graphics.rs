use crate::eadk::{display::*, Color, Rect};
use crate::math::Vec2;
use core::array;

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
    pub fn line(&mut self, start: Vec2, end: Vec2, c: Color) {
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
    pub fn circle(&mut self, center: Vec2, radius: f32, col: Color) {
        let sqd = radius * radius;
        for x in ((center.x - radius) as usize)..((center.x + radius) as usize + 1) {
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
