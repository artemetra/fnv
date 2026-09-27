//! Runs the parser over a directory of real .fnv files.
//! Skipped unless FNV_CORPUS is set, e.g.
//! `FNV_CORPUS=/path/to/_FNV cargo test --test corpus -- --nocapture`
use fnvcli::curve::Fnv;
use fnvcli::file_reader::read_fnv;
use std::path::{Path, PathBuf};

fn fnv_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            fnv_files(&path, out);
        } else if path.extension().map_or(false, |e| e == "fnv") {
            out.push(path);
        }
    }
}

#[test]
fn corpus() {
    let dir = match std::env::var_os("FNV_CORPUS") {
        Some(d) => PathBuf::from(d),
        None => {
            eprintln!("FNV_CORPUS not set, skipping");
            return;
        }
    };
    let mut files = Vec::new();
    fnv_files(&dir, &mut files);
    files.sort();
    assert!(!files.is_empty(), "no .fnv files under {:?}", dir);

    let mut parsed = 0;
    for path in &files {
        let bytes = std::fs::read(path).unwrap();
        match read_fnv(&bytes) {
            Ok(fnv) => {
                // every file that parses must serialize back to identical bytes
                assert_eq!(fnv.to_bytes(), bytes, "round-trip mismatch: {:?}", path);
                // and so must the JSON that `fnvcli show` prints and `fnvcli write` reads
                let json = serde_json::to_string(&fnv).unwrap();
                let back: Fnv = serde_json::from_str(&json).unwrap();
                assert_eq!(back.to_bytes(), bytes, "JSON round-trip mismatch: {:?}", path);
                parsed += 1;
            }
            Err(e) => eprintln!("unparsed {:?}: {}", path.strip_prefix(&dir).unwrap(), e),
        }
    }
    eprintln!("{}/{} files parsed and round-tripped", parsed, files.len());
}
