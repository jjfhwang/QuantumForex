// src/main.rs
/*
 * Main executable for QuantumForex
 */

use clap::Parser;
use quantumforex::{Result, run};

#[derive(Parser)]
#[command(version, about = "QuantumForex - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
