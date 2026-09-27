use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use fnvcli::curve::Fnv;
use fnvcli::file_reader::read_fnv;
use std::collections::BTreeMap;
use std::io::{Read, Write};
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
    /// Write a .fnv file from JSON in the format `show` prints
    Write {
        /// JSON file, or - for stdin
        input: PathBuf,
        /// .fnv file to create, or - for stdout
        output: PathBuf,
        /// Overwrite the output file if it exists
        #[arg(short, long)]
        force: bool,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Show { path } => {
            let bytes = std::fs::read(&path).with_context(|| format!("reading {:?}", path))?;
            let fnv = read_fnv(&bytes).with_context(|| format!("parsing {:?}", path))?;
            let json = serde_json::to_string_pretty(&fnv)? + "\n";
            write_stdout(json.as_bytes())?;
        }
        Command::Scan { dir } => scan(&dir)?,
        Command::Write {
            input,
            output,
            force,
        } => write(&input, &output, force)?,
    }
    Ok(())
}

fn write(input: &Path, output: &Path, force: bool) -> Result<()> {
    let json = if input == Path::new("-") {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).context("reading stdin")?;
        s
    } else {
        std::fs::read_to_string(input).with_context(|| format!("reading {:?}", input))?
    };
    let fnv: Fnv = serde_json::from_str(&json).context("parsing JSON")?;
    fnv.check()?;
    let bytes = fnv.to_bytes();
    // the writer and parser should always agree; don't write a file that doesn't
    read_fnv(&bytes).context("the written file doesn't parse (bug in fnvcli)")?;

    if output == Path::new("-") {
        write_stdout(&bytes)?;
    } else {
        if output.exists() && !force {
            bail!("{:?} already exists, use --force to overwrite it", output);
        }
        std::fs::write(output, &bytes).with_context(|| format!("writing {:?}", output))?;
    }
    Ok(())
}

/// Writes to stdout, stopping quietly if the reader went away (e.g. `| head`).
fn write_stdout(bytes: &[u8]) -> Result<()> {
    let mut out = std::io::stdout().lock();
    match out.write_all(bytes).and_then(|_| out.flush()) {
        Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => Ok(()),
        r => Ok(r?),
    }
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
