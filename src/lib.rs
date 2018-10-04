extern crate alga;
extern crate nalgebra;

use alga::linear::FiniteDimInnerSpace;
use nalgebra::{Real, Vector3, Vector4};

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct LinearPathSegment<V: FiniteDimInnerSpace + Copy, N: Real> {
    pub some_vec: V,
    pub some_scalar: N,
}

#[test]
fn it_works() {
    let vec = Vector3::new(0.0, 1.0, 2.0);
    let vec4 = Vector4::new(0.0, 1.0, 2.0, 3.0);

    let thing = LinearPathSegment {
        some_vec: vec,
        some_scalar: 2.0,
    };

    let thing2 = LinearPathSegment {
        some_vec: vec4,
        some_scalar: 3.0,
    };

    let foo = thing.some_vec * thing.some_scalar;
    let bar = thing2.some_vec / thing2.some_scalar;

    println!("FOO {:?}", foo);
    println!("BAR {:?}", bar);
}
