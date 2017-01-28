use dict::{FstDict, Matrix};

pub const BYTECODE: &'static [u8] = include_bytes!("../../data/ipadic.dic");
pub const MORPHS: &'static [u8] = include_bytes!("../../data/ipadic.morph");
pub const MATRIX: &'static [u8] = include_bytes!("../../data/ipadic.matrix");

pub fn dictionary() -> FstDict<&'static [u8], &'static [i16]> {
    unsafe { FstDict::from_bytes(BYTECODE, MORPHS, MATRIX) }
}

pub fn matrix() -> Matrix<&'static [i16]> {
    unsafe { Matrix::decode(MATRIX) }
}
