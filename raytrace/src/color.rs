// color.rs
use crate::interval::Interval;
use crate::vec3::Vec3;
use std::io::Write;
use std::ops::{Add, AddAssign, Mul};

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        Color::new(value.x, value.y, value.z)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color::new(self * other.r, self * other.g, self * other.b)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

const INTENSITY: Interval = Interval::new(0.0, 0.999);

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let rbyte = (256.0 * INTENSITY.clamp(pixel_color.r)) as usize;
    let gbyte = (256.0 * INTENSITY.clamp(pixel_color.g)) as usize;
    let bbyte = (256.0 * INTENSITY.clamp(pixel_color.b)) as usize;

    writeln!(out, "{rbyte} {gbyte} {bbyte}").unwrap(); // Assume that it works , panic otherwise
}
