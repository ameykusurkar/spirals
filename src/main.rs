#[cfg(feature = "native")]
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use spirals::*;

#[cfg(feature = "native")]
struct Buffer {
    buffer: DynamicImage,
}

#[cfg(feature = "native")]
impl Buffer {
    fn new(height_from_origin: u32) -> Self {
        Self {
            buffer: DynamicImage::new_rgb8(2 * height_from_origin, 2 * height_from_origin),
        }
    }

    fn set(&mut self, x: u32, y: u32, size: u32) {
        for i in 0..size {
            for j in 0..size {
                if self.buffer.in_bounds(x + i, y + j) {
                    self.buffer
                        .put_pixel(x + i, y + j, Rgba([85, 239, 196, 255]));
                }
            }
        }
    }

    fn save(&self, filename: &str) {
        self.buffer.save(filename).expect("Could not save image");
    }
}

#[cfg(feature = "native")]
fn main() {
    let num_points = 10_000_000;
    let mut buffer = Buffer::new(2000);
    let mut generator = PointGenerator::default();

    let start = std::time::Instant::now();
    for _ in 0..num_points {
        let (x, y, n) = generator.next_point();
        let scale = 2000.0 / num_points as f32;
        if is_prime(n) {
            let buffer_x = (x + num_points as f32) * scale;
            let buffer_y = (y + num_points as f32) * scale;
            buffer.set(buffer_x as u32, buffer_y as u32, 1);
        }
    }
    let elapsed = start.elapsed();
    println!("Point generation loop took: {:.2?}", elapsed);

    buffer.save("test.png");
}

#[cfg(not(feature = "native"))]
fn main() {
    panic!("This main function is only available with the 'native' feature enabled");
}
