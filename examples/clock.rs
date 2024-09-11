use std::fs;
use nalgebra::{Affine3, Point3, Rotation3, Scale3, Translation3, Vector2, Vector3};
use gabes_graphics::{
    canvas::Canvas,
    canvas::color::Color,
};
use gabes_graphics::math::affine::Affine3Ext;

fn main() {
    let width = 200;
    let height = 200;
    let mut canvas = Canvas::new(width, height);

    let color = Color::new(0.0, 1.0, 0.2);

    let point = Point3::origin();

    // starting at the top, draw a pixel in clockwise order
    for i in 0..12 {
        use std::f32::consts::PI;

        // move point to 12o'clock position
        let translation = Translation3::new(0.0, 80.0, 0.0);

        // rotate point around origin clockwise
        let axis = -Vector3::z_axis(); // negative z axis, this makes the rotation in the clockwise direction
        let angle = (i as f32) * (2.0 * PI) / 12.0; // divide the circle into 12 sectors
        let rotation = Rotation3::from_axis_angle(&axis, angle);

        // the canvas's (0, 0) is at the top left, increasing y moves down. reflect our point about y
        let reflection_y = Affine3::from_scale(Scale3::new(1.0, -1.0, 1.0));

        // the point still thinks (0, 0) is the origin, move origin to center of canvas
        let to_canvas_center = Translation3::new(width as f32 / 2.0, height as f32 / 2.0, 0.0);

        let point_transformed = (to_canvas_center * reflection_y * rotation * translation) * point;
        // an "Affine Transformation" ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

        let x = point_transformed.x as usize;
        let y = point_transformed.y as usize;

        canvas.set_pixel(x, y, color)
    }


    // write canvas to ppm file
    let ppm = canvas.to_ppm();
    fs::write("clock.ppm", ppm).expect("could not write to file");
}

