use crate::curve::CurveType;
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::slice::Iter;
use nom::{
    bytes::complete::tag,
    IResult,
};
const FNV_FLOAT_MASK: u32 = 0x7000000;
#[derive(Debug)]
pub struct FnvReadError {
    pub kind: FnvReadErrorKind,
}

#[derive(Debug)]
pub enum FnvReadErrorKind {
    /// File size too small to process
    FileTooSmall,
    /// Invalid curve type in file
    InvalidCurveType(u8),
    /// Assertion fail
    AssertionError,
    /// there are 4 known formats for structuring an
    /// fnv file. it seems like it is only possible to
    /// create only one of them by the user, being `03`
    /// so i haven't figure out how to generate/read other
    /// three (`00`, `01`, `02`)
    UnsupportedFormat,
}

impl fmt::Display for FnvReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl fmt::Display for FnvReadErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}

impl Error for FnvReadError {}

// #[macro_export]
// macro_rules! assert_byte_value {
//     ($value:expr, $count:literal, $eq:literal) => {
//         for _ in 0..=$count {
//             match $value {
//                 $eq => Err(FnvReadError{kind: FnvReadErrorKind::AssertionError}),
//                 _ => Ok(()),
//             };
//         }
//     };
// }

pub fn read_fnv_file(_file: &Vec<u8>) -> Result<(), FnvReadError> {
    // smallest size i've found
    if _file.len() < 56 {
        return Err(FnvReadError {
            kind: FnvReadErrorKind::FileTooSmall,
        });
    }
    let mut fi = _file.iter();

    let curve_type = curve_type(fi.next().unwrap())?;
    skip_n_bytes(&mut fi, 3)?;
    assert_parse_type(fi.next().unwrap())?;
    skip_n_bytes(&mut fi, 3)?;
    let point_count: u32 = get_point_count(&vec![
        *fi.next().unwrap(),
        *fi.next().unwrap(),
        *fi.next().unwrap(),
        *fi.next().unwrap(),
        ]);
    skip_n_bytes(&mut fi, 11)?;
    
    Ok(()) // should be removed
}

pub fn curve_type(byte: &u8) -> Result<CurveType, FnvReadError> {
    match byte {
        1 => Ok(CurveType::Envelope),
        2 => Ok(CurveType::Lfo),
        3 => Ok(CurveType::Graph),
        7 => Ok(CurveType::Map),
        n @ _ => Err(FnvReadError {
            kind: FnvReadErrorKind::InvalidCurveType(*n),
        }),
    }
}

// huge thanks to this:
// https://reverseengineering.stackexchange.com/questions/30363/floating-point-number-mangled-in-a-proprietary-file/30364#30364
pub fn read_fnv_float(bytes: u32) -> f32 {
    let f = bytes ^ FNV_FLOAT_MASK;
    let f = swaplow3(f);
    //let f = ror32(f, -3);
    let f = f.rotate_left(3);
    f32::from_bits(f)
}
// for writing floats:
pub fn _write_fnv_float(bytes: u32) -> f32 {
    let f = bytes.rotate_right(3);
    let f = swaplow3(f);
    let f = f ^ FNV_FLOAT_MASK;
    f32::from_bits(f)
}

fn ror32(x: u32, n: i32) -> u32 {
    let n = n & 31;
    let low = x >> n;
    let high = (x << (32 - n)) & 0xFFFFFFFF;
    high | low
}

fn swaplow3(x: u32) -> u32 {
    let x0 = x & 0xFF;
    let x1 = (x >> 8) & 0xFF;
    let x2 = (x >> 16) & 0xFF;
    let x3 = (x >> 24) & 0xFF;
    (x3 << 24) | (x0 << 16) | (x1 << 8) | x2
}

fn assert_zero(x: &u8) -> Result<(), FnvReadError> {
    match x {
        0 => Err(FnvReadError {
            kind: FnvReadErrorKind::AssertionError,
        }),
        _ => Ok(()),
    }
}

fn assert_parse_type(x: &u8) -> Result<(), FnvReadError> {
    match x {
        3 => Ok(()),
        _ => Err(FnvReadError {
            kind: FnvReadErrorKind::UnsupportedFormat,
        }),
    }
}

fn skip_n_bytes(fi: &mut Iter<u8>, n: u8) -> Result<(), FnvReadError> {
    println!("skipping {} bytes", n);
    for _ in 0..n {
        fi.next().unwrap();
    }
    Ok(())
}

pub fn get_point_count(v: &Vec<u8>) -> u32 {
    u32::from_le_bytes(v[..].try_into().unwrap())
}
