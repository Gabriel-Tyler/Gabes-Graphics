# Gabes-Graphics

Run `cargo run --example projectile` to create a path.ppm image file.

Some fun highlights of the code thus far:
- In lib.rs: tests::transformation::chaining::with_chain()
  - A demonstration of chaining transformations
- In math.rs: shear::Affine3Ext
  - Using an extension trait to add functionality to nalgebra::Affine3
