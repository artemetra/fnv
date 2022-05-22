use clap::Parser;
use fnvcli::curve::CurveType;
use fnvcli::file_reader::file_reader;
use std::fs::File;
use std::io;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    println!("{:?}", file_reader::curve_type(&(1 as u8)));
    println!("{:?}", file_reader::curve_type(&(2 as u8)));
    println!("{:?}", file_reader::curve_type(&(3 as u8)));
    println!("{:?}", file_reader::curve_type(&(4 as u8)));
    println!("{:?}", file_reader::curve_type(&(7 as u8)));
}
