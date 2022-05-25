use clap::Parser;
use fnvcli::file_reader::file_reader;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    println!("{}", file_reader::get_point_count(&vec![0x02, 0x00, 0x00, 0x00]));
}
