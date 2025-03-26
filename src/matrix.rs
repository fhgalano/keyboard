use std::vec::Vec;

use zerocopy_derive::{Immutable, IntoBytes, FromBytes};

pub type MatrixLoc = (u8, u8);

#[derive(Debug, Immutable, IntoBytes, FromBytes, PartialEq)]
pub struct MatrixLoc2(pub u8, pub u8);

impl std::convert::From<(u8, u8)> for MatrixLoc2 {
    fn from(value: (u8, u8)) -> Self {
        MatrixLoc2(value.0, value.1)
    }
}

pub trait Matrix {
    fn poll(&mut self) -> Vec<MatrixLoc>;
}
