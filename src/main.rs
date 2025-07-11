// src/main.rs
/*
 * Main executable for BlockchainNFTRegistryKitX
 */

use clap::Parser;
use blockchainnftregistrykitx::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNFTRegistryKitX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
