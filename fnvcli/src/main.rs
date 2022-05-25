use clap::Parser;
use fnvcli::file_reader::file_reader;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    println!("{}", file_reader::read_fnv_float(0x00B571CF));
}
