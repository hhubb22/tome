use std::path::PathBuf;

use crate::cli::WebifyArgs;
use crate::error::{AppError, Result};
use crate::epub::{Epub, site_generator::SiteGenerator};

pub fn run(args: WebifyArgs) -> Result<()> {
    println!("🚀 Webifying EPUB: {}", args.source.display());

    // 1. Open the EPUB file
    let mut epub = Epub::open(&args.source)?;

    // 2. Determine output directory
    let output_dir = get_destination_path(&args.source, args.destination)?;
    println!("   └── Output directory: {}", output_dir.display());

    // 3. Initialize and run the site generator
    // Pass the --no-nav flag to the generator.
    let mut generator = SiteGenerator::new(&mut epub, &output_dir, args.no_nav);
    generator.run()?;

    println!("✅ EPUB successfully converted to a static website.");
    println!("   Find your site in: {}", output_dir.display());
    Ok(())
}

/// Determines the output directory path.
fn get_destination_path(source: &PathBuf, destination: Option<PathBuf>) -> Result<PathBuf> {
    match destination {
        Some(path) => Ok(path),
        None => {
            let stem = source.file_stem()
                .ok_or_else(|| AppError::InvalidSourcePath { path: source.clone() })?;
            let mut new_path = PathBuf::from(stem);
            new_path.set_extension("site"); // e.g., "mybook.epub" -> "mybook.site"
            Ok(new_path)
        }
    }
}