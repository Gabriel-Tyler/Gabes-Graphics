#[cfg(test)]
mod tests {
    mod points_and_vectors {
        use nalgebra as na;
        use approx::assert_relative_eq;

        #[test]
        fn w_coordinate() {
            let p = na::Point3::new(1.0, 2.0, 3.0);
            let v = na::Vector3::new(1.0, 2.0, 3.0);
            assert_relative_eq!(p.to_homogeneous(), na::Vector4::new(1.0, 2.0, 3.0, 1.0));
            assert_relative_eq!(v.to_homogeneous(), na::Vector4::new(1.0, 2.0, 3.0, 0.0));
        }

        #[test]
        fn equality() {
            let p = na::Point3::new(1.0, 2.0, 3.0);
            let v = na::Vector3::new(1.0, 2.0, 3.0);
            assert_relative_eq!(p, na::Point3::new(1.0, 2.0, 3.0));
            assert_relative_eq!(v, na::Vector3::new(1.0, 2.0, 3.0));
        }

        #[test]
        fn addition() {
            let p = na::Point3::new(1.0, 2.0, 3.0);
            let v1 = na::Vector3::new(10.0, 20.0, 30.0);
            let v2 = na::Vector3::new(100.0, 200.0, 300.0);
            assert_relative_eq!(p + v1, na::Point3::new(11.0, 22.0, 33.0));
            assert_relative_eq!(v1 + v2, na::Vector3::new(110.0, 220.0, 330.0));
        }

        #[test]
        fn w_coord_after_addition() {
            let p = na::Point3::new(1.0, 2.0, 3.0);
            let v1 = na::Vector3::new(10.0, 20.0, 30.0);
            let v2 = na::Vector3::new(100.0, 200.0, 300.0);
            assert_relative_eq!((p+v1).to_homogeneous().w, 1.0);
            assert_relative_eq!((v1+v2).to_homogeneous().w, 0.0);
        }

        #[test]
        fn subtraction() {
            let p = na::Point3::new(2.0, 3.0, 4.0);
            let v1 = na::Vector3::new(1.0, 1.0, 1.0);
            let v2 = na::Vector3::new(2.0, 3.0, 4.0);
            assert_relative_eq!(p-v1, na::Point3::new(1.0, 2.0, 3.0));
            assert_relative_eq!(v1-v2, na::Vector3::new(-1.0, -2.0, -3.0));
        }

        #[test]
        fn w_coord_after_subtraction() {
            let p = na::Point3::new(1.0, 2.0, 3.0);
            let v1 = na::Vector3::new(10.0, 20.0, 30.0);
            let v2 = na::Vector3::new(100.0, 200.0, 300.0);
            assert_relative_eq!((p-v1).to_homogeneous().w, 1.0);
            assert_relative_eq!((v1-v2).to_homogeneous().w, 0.0);
        }

        #[test]
        fn negate() {
            let v = na::Vector3::new(1.0, 2.0, 3.0);
            assert_relative_eq!(-v, na::Vector3::new(-1.0, -2.0, -3.0)); // na::Vector impls Neg
            assert_relative_eq!(v, na::Vector3::new(1.0, 2.0, 3.0));
        }

        #[test]
        fn scale_mul() {
            let v = na::Vector3::new(1.0, 2.0, 3.0);
            assert_relative_eq!(v * 10.0, na::Vector3::new(10.0, 20.0, 30.0));
            assert_relative_eq!(v * 0.5, na::Vector3::new(0.5, 1.0, 1.5));
        }

        #[test]
        fn scale_div() {
            let v = na::Vector3::new(10.0, 20.0, 30.0);
            assert_relative_eq!(v / 10.0, na::Vector3::new(1.0, 2.0, 3.0));
        }

        #[test]
        fn point_from_vector() {
            let v = na::Vector3::new(1.0, 2.0, 3.0);
            let p: na::Point3<f64> = v.into();
            assert_relative_eq!(na::Point3::from(v), na::Point3::new(1.0, 2.0, 3.0));
            assert_relative_eq!(p, na::Point3::new(1.0, 2.0, 3.0));
        }

        #[test]
        fn magnitude() {
            let v = na::Vector3::new(1.0, 2.0, 3.0);
            assert_relative_eq!(
                v.magnitude(),
                f64::sqrt(1.0f64.powi(2) + 2.0f64.powi(2) + 3.0f64.powi(2))
            );
        }

        #[test]
        fn normalization() {
            let v = na::Vector3::new(1.0, 2.0, 3.0);
            let u: na::Vector3<f64> = v.normalize();
            assert_relative_eq!(u.magnitude(), 1.0);
        }

        #[test]
        fn dot_product() {
            let v1 = na::Vector3::new(1.0, 2.0, 3.0);
            let v2 = na::Vector3::new(2.0, 3.0, 4.0);
            assert_relative_eq!(v1.dot(&v2), 20.0);
        }

        #[test]
        fn cross_product() {
            let v1 = na::Vector3::new(1.0, 2.0, 3.0);
            let v2 = na::Vector3::new(2.0, 3.0, 4.0);
            assert_relative_eq!(v1.cross(&v2), na::Vector3::new(-1.0, 2.0, -1.0));
        }
    }
}
