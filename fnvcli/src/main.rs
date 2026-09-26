use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use fnvcli::file_reader::read_fnv;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Parse a .fnv file and print it as JSON
    Show {
        path: PathBuf,
    },
    /// Parse every .fnv file under a directory, check that writing it back
    /// gives identical bytes, and print a summary
    Scan {
        dir: PathBuf,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Show { path } => {
            let bytes = std::fs::read(&path).with_context(|| format!("reading {:?}", path))?;
            let fnv = read_fnv(&bytes).with_context(|| format!("parsing {:?}", path))?;
            println!("{}", serde_json::to_string_pretty(&fnv)?);
        }
        Command::Scan { dir } => scan(&dir)?,
    }
    Ok(())
}

fn fnv_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            fnv_files(&path, out)?;
        } else if path.extension().map_or(false, |e| e == "fnv") {
            out.push(path);
        }
    }
    Ok(())
}

fn scan(dir: &Path) -> Result<()> {
    let mut files = Vec::new();
    fnv_files(dir, &mut files)?;
    files.sort();

    let mut by_type = BTreeMap::new();
    let mut failures = Vec::new();
    for path in &files {
        let bytes = std::fs::read(path)?;
        let rel = path.strip_prefix(dir).unwrap_or(path).display().to_string();
        match read_fnv(&bytes) {
            Ok(fnv) if fnv.to_bytes() != bytes => failures.push((rel, "round-trip mismatch".into())),
            Ok(fnv) => *by_type.entry(format!("{:?}", fnv.curve_type)).or_insert(0) += 1,
            Err(e) => failures.push((rel, e.to_string())),
        }
    }

    println!("{} files, {} ok, {} failed", files.len(), files.len() - failures.len(), failures.len());
    for (t, n) in &by_type {
        println!("  {:<8} {}", t, n);
    }
    for (path, err) in &failures {
        println!("FAIL {}: {}", path, err);
    }
    Ok(())
}
