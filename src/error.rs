use std::path::PathBuf;
use thiserror::Error;

// Define a new Result type that uses our custom AppError.
// This makes function signatures throughout the app cleaner.
pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to create directory '{path}': {source}")]
    DirectoryCreation {
        path: PathBuf,
        #[source] // This captures the underlying error (e.g., an io::Error)
        source: std::io::Error,
    },

    #[error("Failed to extract ZIP archive '{path}': {source}")]
    ZipExtraction {
        path: PathBuf,
        #[source]
        source: zip::result::ZipError,
    },

    #[error("Failed to parse XML file '{path}': {source}")]
    XmlParsing {
        path: PathBuf,
        #[source]
        source: quick_xml::DeError,
    },
    
    // New variant for EPUB-specific format errors
    #[error("Invalid EPUB format: {0}")]
    InvalidEpubFormat(String),

    #[error("I/O error: {source}")]
    Io {
        #[from] // Automatically convert from std::io::Error into AppError::Io
        source: std::io::Error,
    },

    #[error("Source file '{path}' has no valid file name")]
    InvalidSourcePath { path: PathBuf },
}