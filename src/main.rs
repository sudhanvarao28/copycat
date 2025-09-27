use std::fs;
use anyhow::Result;
use clap::Parser;
use arboard::Clipboard;
use std::path::PathBuf;

/// Copy what you cat
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Input file to copy to clipboard
    input_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Ensure a file path is provided
    let path = match cli.input_file {
        Some(p) => p,
        None => {
            eprintln!("Error: No input file provided.");
            return Ok(());
        }
    };

    // Read the file as UTF-8 text
    let contents = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("Error reading file {}: {}", path.display(), err);
            return Ok(());
        }
    };

    // Copy to clipboard
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(contents)?;

    println!("File {} copied to clipboard!", path.display());

    Ok(())
}
