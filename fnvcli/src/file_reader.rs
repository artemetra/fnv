// this will be removed... some day
#![allow(dead_code, unused_imports)]
use std::convert::TryInto;
use crate::curve::{CurveType, CurveTrait, GraphCurve};
use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{map, rest_len},
    error::{ErrorKind, ParseError, VerboseError},
    multi::{count, length_data, length_value},
    number::complete::{le_f32, le_u32, le_u8},
    sequence::{pair, preceded, tuple},
    IResult, ToUsize,
};
use std::{error::Error, fmt, slice::Iter};

const FNV_FLOAT_MASK: u32 = 0x7000000;
const MINIMUM_FILESIZE: u32 = 56;

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
    /// three (`00`, `01`, `02`) yet.
    UnsupportedFormat,
}

impl fmt::Display for FnvReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl fmt::Display for FnvReadErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //                  TODO: fix this
        let s = match self {
            Self::FileTooSmall => "FileTooSmall",
            Self::InvalidCurveType(_) => "InvalidCurveType",
            Self::AssertionError => "AssertionError",
            Self::UnsupportedFormat => "UnsupportedFormat",
        };
        write!(f, "{:?}", s)
    }
}

impl Error for FnvReadError {}

fn check_filesize(file: &[u8]) -> IResult<&[u8], &[u8]> {
    take(MINIMUM_FILESIZE)(file)
}

pub fn read_fnv_file(bytes: &'static [u8]) -> Result<impl CurveTrait> {
    let curve_type = read_curve_type(bytes.try_into().unwrap())?;
    let curve_type = match curve_type.0[0] {
        1 => Ok(CurveType::Envelope),
        2 => Ok(CurveType::Lfo),
        3 => Ok(CurveType::Graph),
        7 => Ok(CurveType::Map),
        n @ _ => Err(FnvReadError {
            kind: FnvReadErrorKind::InvalidCurveType(n),
        }),
    }?;
    Ok(GraphCurve::default())
}

// pub fn curve_type(byte: &u32) -> Result<CurveType, FnvReadError> {
//     match byte {
//         1 => Ok(CurveType::Envelope),
//         2 => Ok(CurveType::Lfo),
//         3 => Ok(CurveType::Graph),
//         7 => Ok(CurveType::Map),
//         n @ _ => Err(FnvReadError {
//             kind: FnvReadErrorKind::InvalidCurveType(*n),
//         }),
//     }
// }

fn read_graph_point(bytes: &[u8; 24]) -> IResult<&[u8], &[u8]> {
    take(24_u32)(bytes)
}

fn read_curve_type(bytes: &[u8; 4]) -> IResult<&[u8], (&[u8], &[u8])> {
    pair(
        alt((tag(b"\x01"), tag(b"\x02"), tag(b"\x03"), tag(b"\x07"))),
        tag(b"\x00\x00\x00"),
    )(bytes)
}

// TODO: rename this
fn assert_parse_type(x: &[u8; 4]) -> IResult<&[u8], &[u8]> {
    tag(b"\x03\x00\x00\x00")(x)
}

// huge thanks to this:
// https://reverseengineering.stackexchange.com/questions/30363/floating-point-number-mangled-in-a-proprietary-file/30364#30364
pub fn read_fnv_float(bytes: u32) -> f32 {
    let f = bytes ^ FNV_FLOAT_MASK;
    let f = swaplow3(f);
    let f = f.rotate_left(3);
    f32::from_bits(f)
}
// for writing floats:
pub fn write_fnv_float(fl: f32) -> u32 {
    let f = u32::from_ne_bytes(fl.to_ne_bytes());
    let f = f.rotate_right(3);
    let f = swaplow3(f);
    f ^ FNV_FLOAT_MASK
}

// fn ror32(x: u32, n: i32) -> u32 {
//     let n = n & 31;
//     let low = x >> n;
//     let high = (x << (32 - n)) & 0xFFFFFFFF;
//     high | low
// }

/// Reverses three last bytes.
fn swaplow3(x: u32) -> u32 {
    let x0 = x & 0xFF;
    let x1 = (x >> 8) & 0xFF;
    let x2 = (x >> 16) & 0xFF;
    let x3 = (x >> 24) & 0xFF;
    (x3 << 24) | (x0 << 16) | (x1 << 8) | x2
}
