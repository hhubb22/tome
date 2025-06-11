use std::fs;
use std::io;
use std::path::PathBuf;

use clap::Parser;

/// A simple utility to extract a ZIP archive.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to the ZIP archive file to be extracted.
    #[arg(short, long, value_name = "ZIP_FILE")]
    source: PathBuf,

    /// The destination directory where files will be extracted.
    #[arg(short, long, value_name = "OUTPUT_DIR")]
    destination: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    fs::create_dir_all(&cli.destination)?;

    println!(
        "🚀 start extract: '{}' -> '{}'",
        cli.source.display(),
        cli.destination.display()
    );

    if let Err(e) = extract_zip(&cli.source, &cli.destination) {
        eprintln!("❌ extract failed: {}", e);
        std::process::exit(1);
    }
    
    println!("✅ extract success!");

    Ok(())
}

/// Extracts a ZIP archive to a specified directory.
fn extract_zip(source_path: &PathBuf, dest_path: &PathBuf) -> zip::result::ZipResult<()> {
    let file = fs::File::open(source_path)?;
    let reader = io::BufReader::new(file);

    let mut archive = zip::ZipArchive::new(reader)?;

    archive.extract(dest_path)?;

    Ok(())
}