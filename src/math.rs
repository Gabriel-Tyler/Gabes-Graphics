pub(crate) mod shear {
    use std::ops::Mul;
    use nalgebra::{Matrix4, Point3, Vector, Vector3};

    pub(crate) struct Shear3 {
        matrix: Matrix4<f32>,
    }

    impl Shear3 {
        pub(crate) fn new(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
            Self {
                matrix: Matrix4::new(
                    1.0, xy, xz, 0.0,
                    yx, 1.0, yz, 0.0,
                    zx, zy, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0,
                ),
            }
        }
    }

    /// `Shear3 * Vector3`
    impl Mul<Vector3<f32>> for Shear3 {
        type Output = Vector3<f32>;

        fn mul(self, rhs: Vector3<f32>) -> Self::Output {
            Vector::from_homogeneous(self.matrix * rhs.to_homogeneous())
                .expect("convert to, and back from, a homogeneous vector")
        }
    }

    /// `Shear3 * Point3`
    impl Mul<Point3<f32>> for Shear3 {
        type Output = Point3<f32>;

        fn mul(self, rhs: Point3<f32>) -> Self::Output {
            Point3::from_homogeneous(self.matrix * rhs.to_homogeneous())
                .expect("convert to, and back from, a homogeneous vector")
        }
    }
}