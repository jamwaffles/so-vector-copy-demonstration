extern crate nalgebra;

use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, DimName, Real, VectorN};

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct LinearPathSegment<N: Real, D: DimName>
where
    DefaultAllocator: Allocator<N, D>,
{
    pub some_vec: VectorN<N, D>,
    pub some_scalar: N,
}
