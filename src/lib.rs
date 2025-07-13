use wasm_bindgen::prelude::*;

// Import the `console.log` function from the `console` module
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro to make console.log easier to use
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Default)]
pub struct PointGenerator {
    radius: f32,
    theta: f32,
}

impl PointGenerator {
    pub fn next_point(&mut self) -> (f32, f32, u32) {
        self.radius += 1.0;
        self.theta += 1.0;
        (
            self.radius * self.theta.cos(),
            self.radius * self.theta.sin(),
            self.radius as u32,
        )
    }
}

pub struct PixelBuffer {
    width: u32,
    height: u32,
    data: Vec<u8>, // RGBA format: 4 bytes per pixel
}

impl PixelBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize; // 4 bytes per pixel (RGBA)
        Self {
            width,
            height,
            data: vec![0; size], // Initialize with black/transparent
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        if x < self.width && y < self.height {
            let index = ((y * self.width + x) * 4) as usize;
            if index + 3 < self.data.len() {
                self.data[index] = r; // Red
                self.data[index + 1] = g; // Green
                self.data[index + 2] = b; // Blue
                self.data[index + 3] = a; // Alpha
            }
        }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn into_data(self) -> Vec<u8> {
        self.data
    }
}

pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

pub fn generate_spiral_data(width: u32, height: u32, num_points: u32) -> Vec<u8> {
    let mut buffer = PixelBuffer::new(width, height);
    let mut generator = PointGenerator::default();

    // Use the same scaling approach as the original
    let canvas_size = width.min(height) as f32;
    let scale = canvas_size / (2.0 * num_points as f32);

    for _ in 0..num_points {
        let (x, y, n) = generator.next_point();

        if is_prime(n) {
            // Center the spiral in the canvas and scale appropriately
            let buffer_x = ((x * scale) + canvas_size / 2.0) as u32;
            let buffer_y = ((y * scale) + canvas_size / 2.0) as u32;

            // Use the same color as the original: [85, 239, 196, 255]
            buffer.set_pixel(buffer_x, buffer_y, 85, 239, 196, 255);
        }
    }

    buffer.into_data()
}

// WebAssembly exports
#[wasm_bindgen]
pub fn generate_spiral(width: u32, height: u32, num_points: u32) -> js_sys::Uint8Array {
    let start = js_sys::Date::now();

    let data = generate_spiral_data(width, height, num_points);

    let elapsed = js_sys::Date::now() - start;
    console_log!("Spiral generation took: {:.2}ms", elapsed);

    js_sys::Uint8Array::from(&data[..])
}

#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime_edge_cases() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
    }

    #[test]
    fn test_is_prime_small_primes() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(97), true);
    }

    #[test]
    fn test_is_prime_composites() {
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(25), false);
        assert_eq!(is_prime(100), false);
    }

    #[test]
    fn test_pixel_buffer() {
        let mut buffer = PixelBuffer::new(10, 10);
        buffer.set_pixel(5, 5, 255, 0, 0, 255);

        let data = buffer.get_data();
        let index = (5 * 10 + 5) * 4;
        assert_eq!(data[index], 255); // Red
        assert_eq!(data[index + 1], 0); // Green
        assert_eq!(data[index + 2], 0); // Blue
        assert_eq!(data[index + 3], 255); // Alpha
    }
}
