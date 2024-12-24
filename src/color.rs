use crate::vec3::Vec3;
use crate::utils::clamp;
use std::io::{Write};

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(stream: &mut dyn Write, pixel_color: Color) {
    let Color { x, y, z } = pixel_color;

    let r = linear_to_gamma(x);
    let g = linear_to_gamma(y);
    let b = linear_to_gamma(z);

    const BOUNDS: (f64, f64) = (0.000, 0.999);
    let rbyte = (255.999 * clamp(r, BOUNDS)) as i32;
    let gbyte = (255.999 * clamp(g, BOUNDS)) as i32;
    let bbyte = (255.999 * clamp(b, BOUNDS)) as i32;

    writeln!(stream, "{} {} {}", rbyte, gbyte, bbyte);
}

