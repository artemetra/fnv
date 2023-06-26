use std::convert::TryInto;

use fnvcli::file_reader::{read_fnv_float, write_fnv_float, FnvReadError};
use fnvcli::point::PointMode;
// use clap::Parser;
// #[derive(Parser, Debug)]
// struct Cli {
//     #[clap(parse(from_os_str))]
//     path: std::path::PathBuf,
// }

/// Utility function to test things while copying from HxD.
fn str_to_bytes(s: &str) -> u32 {
    // "12 AB CD EF" -> [0x12, 0xAB, 0xCD, 0xEF]
    let k = s
        .split_whitespace()
        .map(|c| u8::from_str_radix(c, 16).unwrap())
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap_or_else(|v: Vec<u8>| {
            panic!("Expected a Vec of length {} but it was {}", 4, v.len())
        });
    // [0x12, 0xAB, 0xCD, 0xEF] -> 0x12ABCDEF
    u32::from_be_bytes(k)
}

fn main() -> Result<(), FnvReadError> {
    println!(
        "{:x?}",
        write_fnv_float(read_fnv_float(str_to_bytes("C0 07 07 D7")))
    );
    println!("{:?}", PointMode::DoubleCurve);
    Ok(())
}
