use crate::cli::MetaArgs;
use crate::error::Result;
use crate::epub::{model::Metadata, Epub};

pub fn run(args: MetaArgs) -> Result<()> {
    println!("🔍 Analyzing metadata for: {}", args.source.display());

    // The complex logic is now hidden behind `Epub::open`.
    let epub = Epub::open(&args.source)?;

    print_metadata(epub.metadata());

    Ok(())
}

/// Prints metadata in a user-friendly format.
fn print_metadata(metadata: &Metadata) {
    println!("\n--- EPUB Metadata ---");

    if let Some(title) = metadata.title.first() {
        println!("Title:    {}", title);
    }

    let creators = metadata
        .creator
        .iter()
        .map(|c| c.name.as_str())
        .collect::<Vec<&str>>()
        .join(", ");
    if !creators.is_empty() {
        println!("Author(s): {}", creators);
    }

    if let Some(lang) = metadata.language.first() {
        println!("Language: {}", lang);
    }

    if let Some(id) = metadata.identifier.first() {
        println!("Identifier: {}", id.value);
    }

    println!("---------------------\n");
}