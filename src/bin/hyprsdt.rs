//! hyprsdt CLI entry point

use clap::Parser;
use hyprsdt::cli::Cli;

fn main() {
    Cli::parse().run();
}
