//! hyprdt CLI entry point

use clap::Parser;
use hyprdt::cli::Cli;

fn main() {
    Cli::parse().run();
}
