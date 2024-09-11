# Gabes-Graphics

Run `cargo run --example {projectile|clock}` to create a ppm image file in /examples/images/

Some fun highlights of the code thus far:

- In lib.rs: tests::transformation::chaining::with_chain()
    - A demonstration of chaining transformations
- In math.rs: shear::Affine3Ext
    - Using an extension trait to add functionality to nalgebra::Affine3
- In examples/clock.rs: main()
    - Chaining transformations to get a clock shape on the Canvas.
