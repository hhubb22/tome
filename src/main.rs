mod cli;
mod command;
mod epub;
mod error;

use clap::Parser;

use cli::{Cli, Commands};
use anyhow::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Unpack(args) => command::unpack::run(args),
        Commands::Meta(args) => command::meta::run(args), // 新增
    };

    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}