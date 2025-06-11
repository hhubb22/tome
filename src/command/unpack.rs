use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::UnpackArgs;
use crate::error::{AppError, Result};
use crate::epub::Epub;

pub fn run(args: UnpackArgs) -> Result<()> {
    // 1. Determine the destination path
    let dest_path = get_destination_path(&args.source, args.destination)?;
    fs::create_dir_all(&dest_path)
        .map_err(|source| AppError::DirectoryCreation {
            path: dest_path.clone(),
            source,
        })?;

    println!(
        "🚀 Unpacking '{}' -> '{}'",
        args.source.display(),
        dest_path.display()
    );

    // 2. Open the EPUB and unpack it
    let mut epub = Epub::open(&args.source)?;
    epub.unpack_to(&dest_path)?;
    
    println!("✅ Archive extracted successfully.");

    Ok(())
}

/// Determines the output directory path.
fn get_destination_path(source: &Path, destination: Option<PathBuf>) -> Result<PathBuf> {
    match destination {
        Some(path) => Ok(path),
        None => {
            // Default to a new directory named after the source file's stem
            let stem = source.file_stem()
                .ok_or_else(|| AppError::InvalidSourcePath {
                    path: source.to_path_buf(),
                })?;
            Ok(PathBuf::from(stem))
        }
    }
}