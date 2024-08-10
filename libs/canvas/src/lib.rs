mod color;

use color::Color;

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    /// Create a black canvas
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn new() {
        let width = 1920;
        let height = 1080;
        let canvas = Canvas::new(width, height);
        assert_eq!(canvas.width, width);
        assert_eq!(canvas.height, height);
        assert_eq!(canvas.pixels.len(), width * height);
        for i in 0..(width * height) {
            assert_relative_eq!(canvas.pixels[i], Color::new(0.0, 0.0, 0.0));
        }
    }
}
