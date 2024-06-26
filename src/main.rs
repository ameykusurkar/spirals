use image;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

struct Buffer {
    buffer: DynamicImage,
}

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

#[derive(Default)]
struct PointGenerator {
    radius: f32,
    theta: f32,
}

impl PointGenerator {
    fn next_point(&mut self) -> (f32, f32, u32) {
        self.radius += 1.0;
        self.theta += 1.0;
        (
            self.radius * self.theta.cos(),
            self.radius * self.theta.sin(),
            self.radius as u32,
        )
    }
}

fn main() {
    let num_points = 10_000_000;
    let mut buffer = Buffer::new(2000);
    let mut generator = PointGenerator::default();

    for _ in 0..num_points {
        let (x, y, n) = generator.next_point();
        let scale = 2000.0 / num_points as f32;
        if is_prime(n) {
            let buffer_x = (x + num_points as f32) * scale;
            let buffer_y = (y + num_points as f32) * scale;
            buffer.set(buffer_x as u32, buffer_y as u32, 1);
        }
    }

    buffer.save("test.png");
}

fn is_prime(n: u32) -> bool {
    let limit = (n as f64).sqrt() as u32;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }

    true
}
