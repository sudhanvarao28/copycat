use std::{fs::{self}, io::{self, Read}};
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

    let contents = if let Some(path) = cli.input_file {
        fs::read_to_string(&path)?
    }else{
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    // Copy to clipboard
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(contents)?;

    println!("Copied to clipboard!");

    Ok(())
}
