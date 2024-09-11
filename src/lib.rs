mod canvas;
mod math;

pub use canvas::Canvas;
pub use canvas::Color;

#[cfg(test)]
mod tests {
    mod points_and_vectors {
        use nalgebra::{Point3, Vector3, Vector4, Matrix3};
        use approx::assert_relative_eq;

        #[test]
        fn w_coordinate() {
            let p = Point3::new(1.0, 2.0, 3.0);
            let v = Vector3::new(1.0, 2.0, 3.0);
            assert_relative_eq!(p.to_homogeneous(), Vector4::new(1.0, 2.0, 3.0, 1.0));
            assert_relative_eq!(v.to_homogeneous(), Vector4::new(1.0, 2.0, 3.0, 0.0));
        }

        #[test]
        fn equality() {
            let p = Point3::new(1.0, 2.0, 3.0);
            let v = Vector3::new(1.0, 2.0, 3.0);
            assert_relative_eq!(p, Point3::new(1.0, 2.0, 3.0));
            assert_relative_eq!(v, Vector3::new(1.0, 2.0, 3.0));
        }

        #[test]
        fn addition() {
            let p = Point3::new(1.0, 2.0, 3.0);
            let v1 = Vector3::new(10.0, 20.0, 30.0);
            let v2 = Vector3::new(100.0, 200.0, 300.0);
            assert_relative_eq!(p + v1, Point3::new(11.0, 22.0, 33.0));
            assert_relative_eq!(v1 + v2, Vector3::new(110.0, 220.0, 330.0));
        }

        #[test]
        fn w_coord_after_addition() {
            let p = Point3::new(1.0, 2.0, 3.0);
            let v1 = Vector3::new(10.0, 20.0, 30.0);
            let v2 = Vector3::new(100.0, 200.0, 300.0);
            assert_relative_eq!((p+v1).to_homogeneous().w, 1.0);
            assert_relative_eq!((v1+v2).to_homogeneous().w, 0.0);
        }

        #[test]
        fn subtraction() {
            let p = Point3::new(2.0, 3.0, 4.0);
            let v1 = Vector3::new(1.0, 1.0, 1.0);
            let v2 = Vector3::new(2.0, 3.0, 4.0);
            assert_relative_eq!(p-v1, Point3::new(1.0, 2.0, 3.0));
            assert_relative_eq!(v1-v2, Vector3::new(-1.0, -2.0, -3.0));
        }

        #[test]
        fn w_coord_after_subtraction() {
            let p = Point3::new(1.0, 2.0, 3.0);
            let v1 = Vector3::new(10.0, 20.0, 30.0);
            let v2 = Vector3::new(100.0, 200.0, 300.0);
            assert_relative_eq!((p-v1).to_homogeneous().w, 1.0);
            assert_relative_eq!((v1-v2).to_homogeneous().w, 0.0);
        }

        #[test]
        fn negate() {
            // Vector impls std::ops::Neg
            use std::ops::Neg;

            let v = Vector3::new(1.0, 2.0, 3.0);
            assert_relative_eq!(v.neg(), Vector3::new(-1.0, -2.0, -3.0));
            assert_relative_eq!(v, Vector3::new(1.0, 2.0, 3.0));
            assert_relative_eq!(-v, Vector3::new(-1.0, -2.0, -3.0));
            assert_relative_eq!(v, Vector3::new(1.0, 2.0, 3.0));
        }

        #[test]
        fn scale_mul() {
            let v = Vector3::new(1.0, 2.0, 3.0);
            assert_relative_eq!(v * 10.0, Vector3::new(10.0, 20.0, 30.0));
            assert_relative_eq!(v * 0.5, Vector3::new(0.5, 1.0, 1.5));
        }

        #[test]
        fn scale_div() {
            let v = Vector3::new(10.0, 20.0, 30.0);
            assert_relative_eq!(v / 10.0, Vector3::new(1.0, 2.0, 3.0));
        }

        #[test]
        fn point_from_vector() {
            let v = Vector3::new(1.0, 2.0, 3.0);
            let p: Point3<f64> = v.into();
            assert_relative_eq!(Point3::from(v), Point3::new(1.0, 2.0, 3.0));
            assert_relative_eq!(p, Point3::new(1.0, 2.0, 3.0));
        }

        #[test]
        fn magnitude() {
            let v = Vector3::new(1.0, 2.0, 3.0);
            assert_relative_eq!(
                v.magnitude(),
                f64::sqrt(1.0f64.powi(2) + 2.0f64.powi(2) + 3.0f64.powi(2))
            );
        }

        #[test]
        fn normalization() {
            let v = Vector3::new(1.0, 2.0, 3.0);
            let u: Vector3<f64> = v.normalize();
            assert_relative_eq!(u.magnitude(), 1.0);
        }

        #[test]
        fn dot_product() {
            let v1 = Vector3::new(1.0, 2.0, 3.0);
            let v2 = Vector3::new(2.0, 3.0, 4.0);
            assert_relative_eq!(v1.dot(&v2), 20.0);
        }

        #[test]
        fn cross_product() {
            let v1 = Vector3::new(1.0, 2.0, 3.0);
            let v2 = Vector3::new(2.0, 3.0, 4.0);
            assert_relative_eq!(v1.cross(&v2), Vector3::new(-1.0, 2.0, -1.0));
        }

        #[test]
        fn identity_matrix() {
            let mut i = Matrix3::identity();
            let v = Vector3::new(1.0, 2.0, 3.0);
            assert_relative_eq!(i * v, v);
            i[(1, 1)] = 2.0; // changing one element in the identity
            assert_relative_eq!(i * v, Vector3::new(1.0, 4.0, 3.0));
        }
    }

    mod transformation {
        mod translation {
            use approx::assert_relative_eq;
            use nalgebra::{Point3, Translation3, Vector3, Vector4, Matrix4};

            #[test]
            fn point() {
                let t = Translation3::new(5.0, -3.0, 2.0);
                let p = Point3::new(-3.0, 4.0, 5.0);
                assert_relative_eq!(t.transform_point(&p), Point3::new(2.0, 1.0, 7.0));
            }

            #[test]
            fn point_inverse() {
                let t = Translation3::new(5.0, -3.0, 2.0);
                let p = Point3::new(-3.0, 4.0, 5.0);
                assert_relative_eq!(t.inverse_transform_point(&p), Point3::new(-8.0, 7.0, 3.0));
            }

            #[test]
            fn homogeneous() {
                let t = Translation3::new(2.0, 5.0, 8.0);
                assert_relative_eq!(t.to_homogeneous(), Matrix4::new(
                    1.0, 0.0, 0.0, 2.0,
                    0.0, 1.0, 0.0, 5.0,
                    0.0, 0.0, 1.0, 8.0,
                    0.0, 0.0, 0.0, 1.0,
                ));

                let v = Vector3::new(2.0, 3.0, 4.0);
                assert_relative_eq!(v.to_homogeneous(), Vector4::new(2.0, 3.0, 4.0, 0.0));
            }

            #[test]
            fn vector() {
                let t = Translation3::new(5.0, -3.0, 2.0);
                let v = Vector3::new(-3.0, 4.0, 5.0);
                // There is no translation defined for Vectors
                assert_relative_eq!(
                Vector3::from_homogeneous(t.to_homogeneous() * v.to_homogeneous()).unwrap(), v);
            }
        }

        mod scaling {
            use approx::assert_relative_eq;
            use nalgebra::{Scale3, Point3, Matrix4, Vector3};

            #[test]
            fn homogeneous() {
                let s = Scale3::new(2.0, 3.0, 4.0);
                assert_relative_eq!(s.to_homogeneous(), Matrix4::new(
                    2.0, 0.0, 0.0, 0.0,
                    0.0, 3.0, 0.0, 0.0,
                    0.0, 0.0, 4.0, 0.0,
                    0.0, 0.0, 0.0, 1.0
                ));
            }

            #[test]
            fn scale() {
                let s = Scale3::new(2.0, 3.0, 4.0);
                let p = Point3::new(-4.0, 6.0, 8.0);
                let v = Vector3::new(-4.0, 6.0, 8.0);
                assert_relative_eq!(s.transform_point(&p), Point3::new(-8.0, 18.0, 32.0));
                assert_relative_eq!(s * p, Point3::new(-8.0, 18.0, 32.0));
                assert_relative_eq!(s * v, Vector3::new(-8.0, 18.0, 32.0));
            }


            #[test]
            fn inverse() {
                let s = Scale3::new(2.0, 3.0, 4.0);
                let p = Point3::new(-4.0, 6.0, 8.0);
                let v = Vector3::new(-4.0, 6.0, 8.0);
                assert_relative_eq!(s.try_inverse_transform_point(&p).unwrap(), Point3::new(-2.0, 2.0, 2.0));
                assert_relative_eq!(s.try_inverse().unwrap() * v, Vector3::new(-2.0, 2.0, 2.0));
            }

            #[test]
            fn reflection() {
                let s = Scale3::new(-1.0, 1.0, 1.0);
                let p = Point3::new(2.0, 3.0, 4.0);
                let v = Vector3::new(2.0, 3.0, 4.0);
                assert_relative_eq!(s * p, Point3::new(-2.0, 3.0, 4.0));
                assert_relative_eq!(s * v, Vector3::new(-2.0, 3.0, 4.0));
            }
        }

        mod rotation {
            use std::f32;
            use nalgebra::{Point3, Rotation3, Vector3};
            use approx::assert_relative_eq;

            #[test]
            fn about_x_axis() {
                let axis = Vector3::x_axis();
                let half_quarter = f32::consts::FRAC_PI_4;
                let full_quarter = f32::consts::FRAC_PI_2;

                let half_quarter_rot = Rotation3::from_axis_angle(&axis, half_quarter);
                let full_quarter_rot = Rotation3::from_axis_angle(&axis, full_quarter);

                let p = Point3::new(0.0, 1.0, 0.0);
                let v = Vector3::new(0.0, 1.0, 0.0);

                assert_relative_eq!(half_quarter_rot * p, Point3::new(0.0, f32::sqrt(2.0)/2.0, f32::sqrt(2.0)/2.0));
                assert_relative_eq!(full_quarter_rot * p, Point3::new(0.0, 0.0, 1.0));

                assert_relative_eq!(half_quarter_rot * v, Vector3::new(0.0, f32::sqrt(2.0)/2.0, f32::sqrt(2.0)/2.0));
                assert_relative_eq!(full_quarter_rot * v, Vector3::new(0.0, 0.0, 1.0));
            }

            #[test]
            fn inverse_about_x_axis() {
                let axis = Vector3::x_axis();
                let half_quarter = f32::consts::FRAC_PI_4;
                let full_quarter = f32::consts::FRAC_PI_2;

                let half_quarter_rot_inv = Rotation3::from_axis_angle(&axis, half_quarter).inverse();
                let full_quarter_rot_inv = Rotation3::from_axis_angle(&axis, full_quarter).inverse();

                let p = Point3::new(0.0, 1.0, 0.0);
                let v = Vector3::new(0.0, 1.0, 0.0);

                assert_relative_eq!(half_quarter_rot_inv * p, Point3::new(0.0, f32::sqrt(2.0)/2.0, -f32::sqrt(2.0)/2.0));
                assert_relative_eq!(full_quarter_rot_inv * p, Point3::new(0.0, 0.0, -1.0));

                assert_relative_eq!(half_quarter_rot_inv * v, Vector3::new(0.0, f32::sqrt(2.0)/2.0, -f32::sqrt(2.0)/2.0));
                assert_relative_eq!(full_quarter_rot_inv * v, Vector3::new(0.0, 0.0, -1.0));
            }

            #[test]
            fn about_y_axis() {
                let axis = Vector3::y_axis();
                let half_quarter = f32::consts::FRAC_PI_4;
                let full_quarter = f32::consts::FRAC_PI_2;

                let half_quarter_rot = Rotation3::from_axis_angle(&axis, half_quarter);
                let full_quarter_rot = Rotation3::from_axis_angle(&axis, full_quarter);

                let p = Point3::new(0.0, 0.0, 1.0);
                let v = Vector3::new(0.0, 0.0, 1.0);

                assert_relative_eq!(half_quarter_rot * p, Point3::new(f32::sqrt(2.0)/2.0, 0.0, f32::sqrt(2.0)/2.0));
                assert_relative_eq!(full_quarter_rot * p, Point3::new(1.0, 0.0, 0.0));

                assert_relative_eq!(half_quarter_rot * v, Vector3::new(f32::sqrt(2.0)/2.0, 0.0, f32::sqrt(2.0)/2.0));
                assert_relative_eq!(full_quarter_rot * v, Vector3::new(1.0, 0.0, 0.0));
            }

            #[test]
            fn about_z_axis() {
                let axis = Vector3::z_axis();
                let half_quarter = f32::consts::FRAC_PI_4;
                let full_quarter = f32::consts::FRAC_PI_2;

                let half_quarter_rot = Rotation3::from_axis_angle(&axis, half_quarter);
                let full_quarter_rot = Rotation3::from_axis_angle(&axis, full_quarter);

                let p = Point3::new(0.0, 1.0, 0.0);
                let v = Vector3::new(0.0, 1.0, 0.0);

                assert_relative_eq!(half_quarter_rot * p, Point3::new(-f32::sqrt(2.0)/2.0, f32::sqrt(2.0)/2.0, 0.0));
                assert_relative_eq!(full_quarter_rot * p, Point3::new(-1.0, 0.0, 0.0));

                assert_relative_eq!(half_quarter_rot * v, Vector3::new(-f32::sqrt(2.0)/2.0, f32::sqrt(2.0)/2.0, 0.0));
                assert_relative_eq!(full_quarter_rot * v, Vector3::new(-1.0, 0.0, 0.0));
            }
        }

        mod shear {
            use nalgebra::{Point3, Affine3};
            use crate::math::shear::Affine3Ext;
            use approx::assert_relative_eq;

            #[test]
            fn x_from_y() {
                let shear = Affine3::from_shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
                let p = Point3::new(2.0, 3.0, 4.0);

                assert_relative_eq!(shear * p, Point3::new(5.0 , 3.0, 4.0));
            }

            #[test]
            fn x_from_z() {
                let shear = Affine3::from_shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
                let p = Point3::new(2.0, 3.0, 4.0);

                assert_relative_eq!(shear * p, Point3::new(6.0 , 3.0, 4.0));
            }

            #[test]
            fn y_from_x() {
                let shear = Affine3::from_shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
                let p = Point3::new(2.0, 3.0, 4.0);

                assert_relative_eq!(shear * p, Point3::new(2.0 , 5.0, 4.0));
            }

            #[test]
            fn y_from_z() {
                let shear = Affine3::from_shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
                let p = Point3::new(2.0, 3.0, 4.0);

                assert_relative_eq!(shear * p, Point3::new(2.0 , 7.0, 4.0));
            }

            #[test]
            fn z_from_x() {
                let shear = Affine3::from_shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
                let p = Point3::new(2.0, 3.0, 4.0);

                assert_relative_eq!(shear * p, Point3::new(2.0 , 3.0, 6.0));
            }

            #[test]
            fn z_from_y() {
                let shear = Affine3::from_shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
                let p = Point3::new(2.0, 3.0, 4.0);

                assert_relative_eq!(shear * p, Point3::new(2.0 , 3.0, 7.0));
            }
        }

        mod chaining {
            use nalgebra::{Affine3, Point3, Rotation3, Scale3, Translation3, Vector3};
            use approx::assert_relative_eq;
            use crate::math::shear::Affine3Ext;

            #[test]
            fn no_chain() {
                let r = Rotation3::from_axis_angle(&Vector3::x_axis(), std::f32::consts::FRAC_PI_2);
                let sc = Scale3::new(5.0, 5.0, 5.0);
                let t = Translation3::new(10.0, 5.0, 7.0);

                let p1 = Point3::new(1.0, 0.0, 1.0);

                let p2 = r * p1;
                assert_relative_eq!(p2, Point3::new(1.0, -1.0, 0.0));

                let p3 = sc * p2; //                            ≈ 0.0
                assert_relative_eq!(p3, Point3::new(5.0, -5.0, -2.1855695e-7));

                let p4 = t * p3;
                assert_relative_eq!(p4, Point3::new(15.0, 0.0, 7.0));
            }

            #[test]
            fn with_chain() {
                let t = Translation3::new(10.0, 5.0, 7.0);
                let sc = Affine3::from_scale(Scale3::new(5.0, 5.0, 5.0));  // convert to Affine for use in matrix multiplication
                let r = Rotation3::from_axis_angle(&Vector3::x_axis(), std::f32::consts::FRAC_PI_2);
                let sh = Affine3::from_shear(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

                let p = Point3::new(1.0, 0.0, 1.0);

                assert_relative_eq!((t * sc * r * sh) * p, Point3::new(15.0, 0.0, 7.0));
                // Isn't this cool?  ^^^^^^^^^^^^^^^
            }
        }
    }
}

