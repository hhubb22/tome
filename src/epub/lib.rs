// file: epub/lib.rs

use std::fs;
use std::io::{BufReader, Read}; // `copy` is not used here, so remove it.
use std::path::{Path, PathBuf};
use zip::{ZipArchive, read::ZipFile}; // Use `ZipFile` from `zip::read` for clarity
use path_slash::PathBufExt; // Import the extension trait

use crate::error::{AppError, Result};
use crate::epub::model::{Container, Manifest, ManifestItem, Metadata, Package, Spine};

/// Represents an opened EPUB file.
#[derive(Debug)]
pub struct Epub {
    /// The path to the original EPUB file.
    path: PathBuf,
    /// The underlying ZIP archive.
    // Note: The reader type is part of the ZipArchive type signature.
    archive: ZipArchive<BufReader<fs::File>>,
    /// The parsed package data from the .opf file.
    package: Package,
    /// The path to the OPF file inside the archive, crucial for resolving relative paths.
    opf_path: PathBuf,
}

impl Epub {
    pub fn open(path: &Path) -> Result<Self> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let mut archive = ZipArchive::new(reader).map_err(|source| AppError::ZipExtraction {
            path: path.to_path_buf(),
            source,
        })?;

        let opf_path_str = Self::find_opf_path_str(&mut archive)?;
        // We need to clone opf_path_str because `parse_opf` borrows `archive` mutably.
        let package = Self::parse_opf(&mut archive, &opf_path_str)?;
        
        let opf_path = PathBuf::from(opf_path_str);

        Ok(Self {
            path: path.to_path_buf(),
            archive,
            package,
            opf_path,
        })
    }
    
    pub fn metadata(&self) -> &Metadata { &self.package.metadata }
    pub fn manifest(&self) -> &Manifest { &self.package.manifest }
    pub fn spine(&self) -> &Spine { &self.package.spine }

    pub fn unpack_to(&mut self, dest_path: &Path) -> Result<()> {
        self.archive.extract(dest_path)
            .map_err(|source| AppError::ZipExtraction {
                path: self.path.clone(),
                source,
            })
    }

    /// Reads a file from the archive by its manifest item.
    /// The href in the manifest item is relative to the OPF file,
    /// so we need to resolve it.
    // Note the lifetime annotations. We are returning a ZipFile that borrows from `self.archive`.
    pub fn read_by_manifest_item<'a>(&'a mut self, item: &ManifestItem) -> Result<ZipFile<'a, BufReader<fs::File>>> {
        let opf_dir = self.opf_path.parent().unwrap_or_else(|| Path::new(""));

        // Use standard PathBuf and join the path components
        let file_path = opf_dir.join(&item.href);
        
        // Use the `to_slash_lossy()` method from `path_slash` and convert to a String.
        // `to_slash_lossy` returns a `Cow<str>`, `.into_owned()` gives us a `String`.
        let normalized_path = file_path.to_slash_lossy().into_owned();

        self.archive.by_name(&normalized_path)
            .map_err(|e| AppError::ZipExtraction { path: normalized_path.into(), source: e })
    }


    fn find_opf_path_str(archive: &mut ZipArchive<BufReader<fs::File>>) -> Result<String> {
        let mut container_file =
            archive
                .by_name("META-INF/container.xml")
                .map_err(|e| AppError::ZipExtraction {
                    path: "META-INF/container.xml".into(),
                    source: e,
                })?;

        let mut content = String::new();
        container_file.read_to_string(&mut content)?;

        let container: Container =
            quick_xml::de::from_str(&content).map_err(|e| AppError::XmlParsing {
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