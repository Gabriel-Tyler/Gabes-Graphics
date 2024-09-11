pub(crate) mod shear {

    // Transformations are algebraic entities that act on points and vectors.
    // An Affine transformation is a composition of non-uniform scaling, rotations, and translations.
    // Geometrically, it preserves lines and parallelism, but not necessarily Euclidean distances and angles.

    // Shear is an example of an Affine transformation.
    // Shear displaces each point in a fixed direction by an amount proportional to its signed distance
    //  from a given line parallel to that direction.
    // In other words, x is affected by y and z, and vice versa, all at once, as determined by the parameters.

    // I'm always jumping at the chance to use extension traits just because they are cool.
    // Implement an associated method that creates an Affine transformation from shear parameters.
    // You can then use the Affine object in transformation chaining (matrix mult under the hood).
    // e.g., (Affine * Rotation * Translation) * Vector

    use nalgebra::{Affine3, Matrix, Matrix4, RealField, Scale3};

    pub(crate) trait Affine3Ext<T: RealField> {
        fn from_shear(xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Self;
        fn from_scale(scale: Scale3<T>) -> Self;
    }

    impl<T> Affine3Ext<T> for Affine3<T>
    where
        T: RealField,
    {
        fn from_shear(xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Self {
            Affine3::from_matrix_unchecked(
                Matrix4::new(
                    // Can't use 1.0 or 0.0 b/c of generic type mismatch.
                    // Can't make one and zero variables b/c of borrow checker
                    T::one(), xy, xz, T::zero(),
                    yx, T::one(), yz, T::zero(),
                    zx, zy, T::one(), T::zero(),
                    T::zero(), T::zero(), T::zero(), T::one(),
                )
            )
        }

        fn from_scale(scale: Scale3<T>) -> Self {
            Affine3::from_matrix_unchecked(scale.to_homogeneous())
        }
    }
}

