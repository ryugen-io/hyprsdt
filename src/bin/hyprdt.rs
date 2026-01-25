//! hyprdt CLI - Debug terminal server

use clap::Parser;
use hyprdt::Server;
use hyprdt::socket::{Level, default_socket_path};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "hyprdt",
    version,
    about = "Debug terminal for hypr* ecosystem - real-time log viewer"
)]
struct Args {
    /// Socket path (default: $XDG_RUNTIME_DIR/hyprdt.sock)
    #[arg(short, long)]
    socket: Option<PathBuf>,

    /// Filter logs to specific app name
    #[arg(short, long)]
    app: Option<String>,

    /// Minimum log level (error, warn, info, debug, trace)
    #[arg(short, long)]
    level: Option<String>,

    /// Hide timestamps
    #[arg(long)]
    no_timestamps: bool,
}

fn main() {
    let args = Args::parse();

    let socket_path = args.socket.unwrap_or_else(default_socket_path);
    let cleanup_path = socket_path.clone();

    let min_level = args.level.map(|l| Level::parse(&l));

    let config = hyprdt::server::ServerConfig {
        socket_path,
        app_filter: args.app,
        min_level,
        show_timestamps: !args.no_timestamps,
    };

    let server = Server::with_config(config);

    // Handle Ctrl+C - clean up socket before exit
    let _ = ctrlc::set_handler(move || {
        let _ = std::fs::remove_file(&cleanup_path);
        std::process::exit(0);
    });

    if let Err(e) = server.run() {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
