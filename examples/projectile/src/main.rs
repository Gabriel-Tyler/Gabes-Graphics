use nalgebra as na;
use lib_canvas::{Canvas, Color};

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

    fn is_above_ground(&self) -> bool {
        self.position.y > 0.0
    }
}

impl Environment {
    fn new(gravity: na::Vector2<f64>, wind: na::Vector2<f64>) -> Self {
        Self {
            gravity,
            wind,
        }
    }
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}

fn main() {
    // projectile starts one unit above the origin.
    // velocity is normalized to 1 unit/tick.
    //   p ← projectile(point(0, 1, 0), normalize(vector(1, 1, 0)))
    let mut p = Projectile::new(
        na::Point2::new(0.0, 1.0),
        na::Vector2::new(1.0, 1.0).normalize(),
    );
    // gravity -0.1 unit/tick, and wind is -0.01 unit/tick.
    //   e ← environment(vector(0, -0.1, 0), vector(-0.01, 0, 0))
    let e = Environment::new(
        na::Vector2::new(0.0, -1.0),
        na::Vector2::new(-0.01, 0.0),
    );

    let c = Canvas::new(3, 3);
    c.set_pixel(p.position.x, p.position.y, Color::new(0.9, 0.0, 0.1));

    while p.is_above_ground() {
        p = tick(&e, p);
        c.set_pixel(p.position.x, p.position.y, Color::new(0.9, 0.0, 0.1));
    }
}


