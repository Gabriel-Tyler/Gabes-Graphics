#[cfg(test)]
mod tests {
    // use super::*;
    use nalgebra as na;
    use approx::assert_relative_eq;

    #[test]
    fn point_and_vector_w_coordinate() {
        let p = na::Point3::new(1.0, 2.0, 3.0);
        let v = na::Vector3::new(1.0, 2.0, 3.0);
        assert_relative_eq!(p.to_homogeneous(), na::Vector4::new(1.0, 2.0, 3.0, 1.0));
        assert_relative_eq!(v.to_homogeneous(), na::Vector4::new(1.0, 2.0, 3.0, 0.0));
    }
}
