// color.rs
use std::io::Write;

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

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let rbyte = (255.999 * pixel_color.r) as usize;
    let gbyte = (255.999 * pixel_color.g) as usize;
    let bbyte = (255.999 * pixel_color.b) as usize;

    // Could return an error
    writeln!(out, "{rbyte} {gbyte} {bbyte}").unwrap(); // Assume that it works , panic otherwise

    // let result = writeln!(out, "{rbyte} {gbyte} {bbyte}")
    // match result {
    //     Ok(r) => {
    //         //it worked
    //     }
    //     Err(e) => {
    //         panic!("{e:?}")
    //     }
    // }
}
