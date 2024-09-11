use std::fs;
use nalgebra as na;
use gabes_graphics::{
    canvas::Canvas,
    canvas::color::Color,
};

#[derive(Clone, Debug, PartialEq)]
struct Projectile {
    position: na::Point2<f64>,
    velocity: na::Vector2<f64>,
}

#[derive(Clone, Debug, PartialEq)]
struct Environment {
    gravity: na::Vector2<f64>,
    wind: na::Vector2<f64>,
}

impl Projectile {
    fn new(position: na::Point2<f64>, velocity: na::Vector2<f64>) -> Self {
        Self {
            position,
            velocity,
        }
    }

    fn is_in_view(&self, width: usize, height: usize) -> bool {
        self.position.x >= 0.0
            && self.position.x < width as f64
            && self.position.y >= 0.0
            && self.position.y < height as f64
    }

    /// Is the projectile moving?
    fn has_velocity(&self) -> bool {
        self.velocity.magnitude() > 0.0
    }

    fn x(&self) -> usize {
        self.position.x as usize
    }

    fn y(&self) -> usize {
        self.position.y as usize
    }
}

impl Environment {
    fn new(gravity: na::Vector2<f64>, wind: na::Vector2<f64>) -> Self {
        Self {
            gravity,
            wind,
        }
    }

    /// Does the environment affect velocity?
    fn has_acceleration(&self) -> bool {
        (self.gravity + self.wind).magnitude() > 0.0
    }
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}

fn main() {
    // Projectile starts one unit above the origin.
    // Velocity is normalized to 11.25 units/tick.
    let mut p = Projectile::new(
        na::Point2::new(0.0, 1.0),
        na::Vector2::new(1.0, 1.8).normalize() * 11.25,
    );

    // Gravity -0.1 unit/tick, and wind is -0.01 unit/tick.
    let e = Environment::new(
        na::Vector2::new(0.0, -0.1),
        na::Vector2::new(-0.01, 0.0),
    );

    let width = 900;
    let height = 550;
    let mut c = Canvas::new(width, height);

    // Must check for velocity and acceleration.
    // Otherwise, the projectile may get stuck inside the view and cause an infinite loop.
    while p.is_in_view(width, height) && (p.has_velocity() || e.has_acceleration()) {
        c.set_pixel(p.x(), height - 1 - p.y(), Color::new(0.0, 1.0, 0.1));
        p = tick(&e, p);
    }

    // Write ppm formatted data to string then to file.
    let ppm = c.to_ppm();
    fs::write("path.ppm", ppm).expect("could not write to file");
}


