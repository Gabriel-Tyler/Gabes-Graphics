pub mod color;

use color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    /// Create a black canvas
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    /// Draw a color at the given coordinate
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        assert!(x < self.width && y < self.height);

        self.pixels[x + y * self.width] = color;
    }

    /// Get the color at the given coordinate
    fn get_pixel(&self, x: usize, y: usize) -> Color {
        assert!(x < self.width && y < self.height);

        self.pixels[x + y * self.width]
    }

    /// Write the contents of the canvas to a PPM file
    pub fn to_ppm(&self) -> String {
        let max_color_value = 255;
        let header = format!("P3\n{} {}\n{}\n", self.width, self.height, max_color_value);
        let mut data = String::from(header);
        for i in 0..self.height {
            for j in 0..self.width {
                let c = self.get_pixel(j, i);
                let (r, g, b) = c.scale(max_color_value);
                data.push_str(&format!("{r} {g} {b}\n"));
            }
        }
        data
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

    #[test]
    fn get_set() {
        let width = 3;
        let height = 2;
        let mut canvas = Canvas::new(width, height);
        assert_relative_eq!(canvas.get_pixel(1, 0), Color::new(0.0, 0.0, 0.0));
        canvas.set_pixel(1, 0, Color::new(0.0, 0.5, 1.0));
        assert_relative_eq!(canvas.get_pixel(1, 0), Color::new(0.0, 0.5, 1.0));
        assert_relative_eq!(canvas.get_pixel(0, 0), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn to_ppm() {
        let mut canvas = Canvas::new(3, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-2.0, 0.0, 1.0);
        canvas.set_pixel(0, 0, c1);
        canvas.set_pixel(1, 1, c2);
        canvas.set_pixel(2, 2, c3);

        let ppm = canvas.to_ppm();
        assert_eq!(
            ppm,
            //   w h  max  row1_________________  row2_________________  row3_________________
            "P3\n3 3\n255\n255 0 0\n0 0 0\n0 0 0\n0 0 0\n0 128 0\n0 0 0\n0 0 0\n0 0 0\n0 0 255\n"
        );
    }

    #[test]
    fn ppm_has_newline() {
        // Some image programs require ppm files to end in a newline
        let canvas = Canvas::new(1920, 1080);
        let ppm = canvas.to_ppm();
        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }
}
