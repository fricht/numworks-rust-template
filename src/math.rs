use core::ops::{Add, Mul, Sub};

use trigo::{cos, sin};

//// list object
// i cant bc i need to do unsafe things (which mean memory leak)

/// 2-dimentional vector (integers)
#[derive(Default, Clone, Copy)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

/// element-wise multiplication (by integer)
impl Mul<i32> for Vec2i {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Vec2i {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// element-wise multiplication (by float)
impl Mul<f32> for Vec2i {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x as f32 * rhs,
            y: self.y as f32 * rhs,
        }
    }
}

/// element-wise addition
impl Add<Vec2i> for Vec2i {
    type Output = Self;
    fn add(self, rhs: Vec2i) -> Self::Output {
        Vec2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// element-wise substraction
impl Sub<Vec2i> for Vec2i {
    type Output = Self;
    fn sub(self, rhs: Vec2i) -> Self::Output {
        Vec2i {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Vec2i {
    /// get the norm (or the length) of the vector
    pub fn norm(self) -> f32 {
        sqrt((self.x * self.x + self.y * self.y) as f32)
    }

    /// return a normalized copy of the vector
    pub fn normalized(self) -> Vec2 {
        let inv_sqrt_norm = inv_sqrt((self.x * self.x + self.y * self.y) as f32);
        Vec2 {
            x: self.x as f32 * inv_sqrt_norm,
            y: self.y as f32 * inv_sqrt_norm,
        }
    }

    /// return the dot product of the vectors
    pub fn dot(self, other: Self) -> i32 {
        self.x * other.x + self.y * other.y
    }

    /// save vector with floats
    pub fn to_float(self) -> Vec2 {
        Vec2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

/// 2-dimensional vector (floats)
#[derive(Default, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// element-wise multiplication
impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// element-wise addition
impl Add<Vec2> for Vec2 {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// element-wise substraction
impl Sub<Vec2> for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Vec2 {
    /// get the norm (or the length) of the vector
    pub fn norm(self) -> f32 {
        sqrt(self.x * self.x + self.y * self.y)
    }

    /// normalize the vector
    pub fn normalize(&mut self) {
        let inv_sqrt_norm = inv_sqrt(self.x * self.x + self.y * self.y);
        self.x = self.x * inv_sqrt_norm;
        self.y = self.y * inv_sqrt_norm;
    }

    /// return a normalized copy of the vector
    pub fn normalized(self) -> Self {
        let inv_sqrt_norm = inv_sqrt(self.norm());
        Vec2 {
            x: self.x * inv_sqrt_norm,
            y: self.y * inv_sqrt_norm,
        }
    }

    /// rotate the vector
    pub fn rotate(&mut self, angle: f32) {
        let cosx = cos(angle);
        let sinx = sin(angle);
        let x = self.x * cosx - self.y * sinx;
        let y = self.x * sinx + self.y * cosx;
        self.x = x;
        self.y = y;
    }

    /// return a normalized copy of the vector
    pub fn rotated(self, angle: f32) -> Self {
        let cosx = cos(angle);
        let sinx = sin(angle);
        Vec2 {
            x: self.x * cosx - self.y * sinx,
            y: self.x * sinx + self.y * cosx,
        }
    }

    /// return the dot product of the vectors
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// return a new vector linearly interpolated to a target vector
    pub fn lerp(self, target: Self, t: f32) -> Self {
        self + (target - self) * t
    }

    /// get the angle from the vector to (1, 0)
    pub fn get_angle(&self) -> f32 {
        unimplemented!()
    }

    /// tangent angle
    pub fn tan_angle(&self) -> f32 {
        self.y / self.x
    }

    /// inverse tangent angle
    pub fn inv_tan_angle(&self) -> f32 {
        self.x / self.y
    }

    /// floor each component
    pub fn floor(&self) -> Vec2i {
        Vec2i {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

/// Quake's Fast Inverse Square Root
pub fn inv_sqrt(x: f32) -> f32 {
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);
    y * (1.5 - 0.5 * x * y * y)
}

/// sqrt derived from fast inv sqrt
pub fn sqrt(x: f32) -> f32 {
    1. / inv_sqrt(x)
}

/// trigonometry module
pub mod trigo {
    use core::f32;

    /// 5th degree polynomial interpolation of the first quarter (of 2pi) of the cosine function
    fn quarter_cos(x: f32) -> f32 {
        1.00002 - 0.000447247 * x - 0.497081 * x * x - 0.00767104 * x * x * x
            + 0.0512404 * x * x * x * x
            - 0.00575391 * x * x * x * x * x
    }

    /// cosine function (approx)
    pub fn cos(x: f32) -> f32 {
        // transformations
        let mut x = ((x % f32::consts::TAU) + f32::consts::TAU) % f32::consts::TAU; // bc rust modulo can be neg ):<
        if x > f32::consts::PI {
            x = f32::consts::TAU - x;
        }
        let mut multiplier = 1.;
        if x > f32::consts::FRAC_PI_2 {
            multiplier = -1.;
            x = f32::consts::PI - x;
        }
        //sample
        quarter_cos(x) * multiplier
    }

    /// sine function (approx)
    pub fn sin(x: f32) -> f32 {
        cos(x - f32::consts::FRAC_PI_2)
    }

    /// tangent function (approx)
    pub fn tan(x: f32) -> f32 {
        sin(x) / cos(x)
    }

    /// inverse tangent (approx)
    pub fn inv_tan(x: f32) -> f32 {
        cos(x) / sin(x)
    }

    // TODO : other approx (taylor ?)
    // https://www.desmos.com/calculator/ymtgipxmdg
    /// atan function (please don't kill me)
    pub fn atan(x: f32) -> f32 {
        if x >= 0. {
            -1. / (0.636 * x + f32::consts::FRAC_2_PI) + f32::consts::FRAC_PI_2
        } else {
            -1. / (0.636 * x - f32::consts::FRAC_2_PI) - f32::consts::FRAC_PI_2
        }
    }
}

/// step function (range [0, 1])
pub fn step(x: f32, threshold: f32) -> f32 {
    if x < threshold {
        0.
    } else {
        1.
    }
}

/// step function (range [-1, 1])
pub fn large_step(x: f32, threshold: f32) -> f32 {
    if x < threshold {
        -1.
    } else {
        1.
    }
}
