use std::convert::AsRef;
use std::io::{self, Write};
use std::ops::{Index, IndexMut};

use byteorder::{NativeEndian, WriteBytesExt, ByteOrder};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T: AsRef<[i16]>> {
    width: u16,
    height: u16,
    table: T,
}

impl Matrix<Vec<i16>> {
    pub fn with_zeros(width: u16, height: u16) -> Self {
        Matrix {
            width: width,
            height: height,
            table: vec![0; width as usize * height as usize],
        }
    }
}

impl<T: AsRef<[i16]>> Matrix<T> {
    pub fn encode<W: Write, O: ByteOrder>(&self, mut w: W) -> io::Result<()> {
        w.write_u16::<O>(self.width)?;
        w.write_u16::<O>(self.height)?;
        for &byte in self.table.as_ref() {
            w.write_i16::<O>(byte)?;
        }
        Ok(())
    }

    pub fn encode_native<W: Write>(&self, w: W) -> io::Result<()> {
        self.encode::<W, NativeEndian>(w)
    }

    pub fn connection_cost(&self, right_id: u16, left_id: u16) -> i16 {
        self[(right_id, left_id)]
    }
}

impl<'a> Matrix<&'a [i16]> {
    pub unsafe fn decode(bs: &'a [u8]) -> Self {
        let ptr = bs.as_ptr() as *const u16;
        let width = *ptr;
        let height = *ptr.offset(1);
        let ptr = ptr.offset(2) as *const i16;
        let table = ::std::slice::from_raw_parts(ptr, width as usize * height as usize);
        Matrix {
            width: width,
            height: height,
            table: table,
        }
    }
}

impl<T: AsRef<[i16]>> Index<(u16, u16)> for Matrix<T> {
    type Output = i16;
    fn index(&self, index: (u16, u16)) -> &i16 {
        let w = index.0 as usize;
        let h = index.1 as usize;
        &self.table.as_ref()[w + h * self.width as usize]
    }
}

impl IndexMut<(u16, u16)> for Matrix<Vec<i16>> {
    fn index_mut(&mut self, index: (u16, u16)) -> &mut i16 {
        let w = index.0 as usize;
        let h = index.1 as usize;
        &mut self.table[w + h * self.width as usize]
    }
}

#[test]
fn test_encode_decode() {
    let table: &[i16] = &[-3, -2, -1, 0, 1, 2];
    let matrix = Matrix {
        width: 2,
        height: 3,
        table: table,
    };
    let mut buf = Vec::new();
    matrix.encode_native(&mut buf).unwrap();
    let decoded = unsafe { Matrix::decode(&buf) };
    assert_eq!(decoded, matrix);
}
