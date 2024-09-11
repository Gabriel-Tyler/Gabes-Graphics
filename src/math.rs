pub(crate) mod shear {

    // Transformations are algebraic entities that act on points and vectors.
    // An Affine transformation is a composition of non-uniform scaling, rotations, and translations.
    // Geometrically, it preserves lines and parallelism, but not necessarily Euclidean distances and angles.

    // I'm always jumping at the chance to use extension traits just because they are cool.
    // Implement an associated method that creates an Affine transformation from shear parameters.
    // You can then use the Affine object in transformation chaining (matrix mult under the hood).
    // e.g., (Affine * Rotation * Translation) * Vector

    use nalgebra::{Affine3, Matrix4, RealField};

    pub(crate) trait Affine3Ext<T: RealField> {
        fn from_shear(xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Self;
    }

    impl<T: RealField> Affine3Ext<T> for Affine3<T> {
        fn from_shear(xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Self {
            Affine3::from_matrix_unchecked(
                Matrix4::new(
                    1.0, xy, xz, 0.0,
                    yx, 1.0, yz, 0.0,
                    zx, zy, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0,
                )
            )
        }
    }
}

