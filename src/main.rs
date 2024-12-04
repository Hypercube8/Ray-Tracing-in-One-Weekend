mod vec3;
mod color;

use color::{Color, write_color};

fn main() {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        eprintln!("Scanlines remaining: {}", IMAGE_HEIGHT-j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                i as f64 / (IMAGE_WIDTH-1) as f64,
                j as f64 / (IMAGE_HEIGHT-1) as f64,
                0.0
            );
            write_color(pixel_color);
        }
    }

    eprintln!("Done.");
}
