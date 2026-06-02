use alloc::vec::Vec;

use zerocopy_derive::{Immutable, IntoBytes, FromBytes};


#[derive(
    Eq,
    Hash,
    Clone, 
    Debug, 
    Immutable, 
    IntoBytes, 
    FromBytes, 
    PartialEq,
)]
pub struct MatrixLoc(pub u8, pub u8);

impl core::convert::From<(u8, u8)> for MatrixLoc {
    fn from(value: (u8, u8)) -> Self {
        MatrixLoc(value.0, value.1)
    }
}

pub trait Matrix {
    fn poll(&mut self) -> Vec<MatrixLoc>;
}

pub struct PhantomMatrix;

impl PhantomMatrix {
    pub fn new() -> Self {
        Self
    }
}

impl Matrix for PhantomMatrix {
    fn poll(&mut self) -> Vec<MatrixLoc> {
        Vec::new()
    }
}
