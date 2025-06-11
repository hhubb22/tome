// epub/mod.rs
pub mod model;

use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use zip::{ZipArchive, result::ZipError};

use crate::error::{AppError, Result};
use model::{Container, Package, Metadata};

/// Represents an opened EPUB file.
/// This struct provides a high-level API for interacting with the EPUB's contents.
#[derive(Debug)]
pub struct Epub {
    /// The path to the original EPUB file.
    path: PathBuf,
    /// The underlying ZIP archive.
    archive: ZipArchive<BufReader<fs::File>>,
    /// The parsed package data from the .opf file.
    package: Package,
}

impl Epub {
    /// Opens an EPUB file and parses its essential metadata structures.
    pub fn open(path: &Path) -> Result<Self> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let mut archive = ZipArchive::new(reader).map_err(|source| AppError::ZipExtraction {
            path: path.to_path_buf(),
            source,
        })?;

        let opf_path = Self::find_opf_path(&mut archive)?;
        let package = Self::parse_opf(&mut archive, &opf_path)?;

        Ok(Self {
            path: path.to_path_buf(),
            archive,
            package,
        })
    }

    /// Returns a reference to the EPUB's metadata.
    pub fn metadata(&self) -> &Metadata {
        &self.package.metadata
    }


    /// Extracts the entire EPUB archive to a specified directory.
    pub fn unpack_to(&mut self, dest_path: &Path) -> Result<()> {
        self.archive.extract(dest_path)
            .map_err(|source| AppError::ZipExtraction {
                path: self.path.clone(),
                source,
            })
    }

    /// In-memory search for container.xml to find the .opf file path.
    fn find_opf_path(archive: &mut ZipArchive<BufReader<fs::File>>) -> Result<String> {
        let mut container_file = archive
            .by_name("META-INF/container.xml")
            .map_err(|e| {
                if let ZipError::FileNotFound = e {
                    AppError::InvalidEpubFormat("META-INF/container.xml not found".to_string())
                } else {
                    AppError::ZipExtraction {
                        path: "META-INF/container.xml".into(),
                        source: e,
                    }
                }
            })?;

        let mut content = String::new();
        container_file.read_to_string(&mut content)?;

        let container: Container = quick_xml::de::from_str(&content).map_err(|e| AppError::XmlParsing {
            path: "META-INF/container.xml".into(),
            source: e,
        })?;

        container
            .rootfiles
            .rootfile
            .first()
            .map(|rf| rf.full_path.clone())
            .ok_or_else(|| AppError::InvalidEpubFormat("No rootfile found in container.xml".to_string()))
    }

    /// Reads and parses the .opf file from the archive.
    fn parse_opf(archive: &mut ZipArchive<BufReader<fs::File>>, opf_path: &str) -> Result<Package> {
         let mut opf_file = archive.by_name(opf_path).map_err(|e| AppError::ZipExtraction {
            path: opf_path.to_string().into(),
            source: e,
        })?;

        let mut opf_content = String::new();
        opf_file.read_to_string(&mut opf_content)?;

        quick_xml::de::from_str(&opf_content).map_err(|e| AppError::XmlParsing {
            path: opf_path.to_string().into(),
            source: e,
        })
    }
}