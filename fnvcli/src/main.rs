use clap::Parser;
use fnvcli::file_reader::{self, FnvReadError};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), FnvReadError> {
    // println!("{:?}", file_reader::parser(&[0x00, 0xFF, 0xAA]));
    Ok(())
}
