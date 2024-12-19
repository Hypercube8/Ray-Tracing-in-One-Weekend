use crate::vec3::Vec3;
use crate::utils::clamp;
use std::io::{Write};

pub type Color = Vec3;

pub fn write_color(stream: &mut dyn Write, pixel_color: Color) {
    let Color { x, y, z } = pixel_color;

    const BOUNDS: (f64, f64) = (0.000, 0.999);
    let r = (255.999 * clamp(x, BOUNDS)) as i32;
    let g = (255.999 * clamp(y, BOUNDS)) as i32;
    let b = (255.999 * clamp(z, BOUNDS)) as i32;

    writeln!(stream, "{} {} {}", r, g, b);
}

