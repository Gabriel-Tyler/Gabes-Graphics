use std::ops::{Add, Mul, Sub};
use approx::{AbsDiffEq, RelativeEq};
use nalgebra as na;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Color {
    rgb: na::Vector3<f32>,
}

impl Color {
    pub(crate) fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            rgb: na::Vector3::new(r, g, b),
        }
    }

    pub(crate) fn r(&self) -> f32 {
        self.rgb.x
    }

    pub(crate) fn g(&self) -> f32 {
        self.rgb.y
    }

    pub(crate) fn b(&self) -> f32 {
        self.rgb.z
    }
}

/// `Color + Color`
impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { rgb: self.rgb + rhs.rgb }
    }
}

/// `Color - Color`
impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { rgb: self.rgb - rhs.rgb }
    }
}

/// `Color * Color`
///
/// For blending colors together
impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        // rgb = (r*r, g*g, b*b)
        Self { rgb: self.rgb.component_mul(&rhs.rgb) }
    }
}

/// `Color * f32`
impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self { rgb: self.rgb * rhs }
    }
}

/// `f32 * Color`
impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color { rgb: rhs.rgb * self }
    }
}

/// Required for `RelativeEq`
impl AbsDiffEq for Color {
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        f32::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.rgb.abs_diff_eq(&other.rgb, epsilon)
    }
}

/// Required for `assert_relative_eq!(Color, Color)`
impl RelativeEq for Color {
    fn default_max_relative() -> Self::Epsilon {
        f32::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        self.rgb.relative_eq(&other.rgb, epsilon, max_relative)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn new() {
        let c = Color::new(0.0, 0.5, 1.0);
        assert_relative_eq!(c.r(), 0.0);
        assert_relative_eq!(c.g(), 0.5);
        assert_relative_eq!(c.b(), 1.0);
        assert_relative_eq!(c, Color::new(0.0, 0.5, 1.0));
    }

    #[test]
    fn add() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_relative_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn sub() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_relative_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn mul() {
        // `Color * f32` and `f32 * Color`
        let c = Color::new(0.2, 0.3, 0.4);
        assert_relative_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
        assert_relative_eq!(3.0 * c, Color::new(0.6, 0.9, 1.2));

        // `Color * Color` using component-wise multiplication
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_relative_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}