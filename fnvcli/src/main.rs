use clap::Parser;
use fnvcli::file_reader::{self, FnvReadError};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), FnvReadError> {
    let f = std::fs::read("oh god.fnv").unwrap();
    file_reader::read_fnv_file(&f)?;
    Ok(())
}
