use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};

/// A toolkit for working with EPUB files.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Unpacks an EPUB file into a specified directory.
    Unpack(UnpackArgs),
    // You can add other subcommands here later
    // Pack(PackArgs),
     Meta(MetaArgs),
}

#[derive(Args, Debug)]
pub struct UnpackArgs {
    /// The path to the source EPUB file.
    #[arg(required = true)]
    pub source: PathBuf,

    /// The destination directory for extracted files.
    /// If omitted, a directory with the same name as the EPUB file will be created.
    #[arg(short, long, value_name = "OUTPUT_DIR")]
    pub destination: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct MetaArgs {
    /// The path to the source EPUB file.
    #[arg(required = true)]
    pub source: PathBuf,
}