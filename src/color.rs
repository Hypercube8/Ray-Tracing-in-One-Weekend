use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    let Color { x, y, z } = pixel_color;

    let r = (255.999 * x) as i32;
    let g = (255.999 * y) as i32;
    let b = (255.999 * z) as i32;

    println!("{} {} {}", r, g, b);
}

