#![allow(unused)]

use anyhow::{Context, Result};
use clap::{Error, Parser};
use std::fs::File;
use std::io::BufReader;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.to_string_lossy()))?;
    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}
