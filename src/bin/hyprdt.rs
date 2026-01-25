//! hyprdt CLI - Debug terminal server

use clap::Parser;
use hyprdt::socket::Level;
use hyprdt::{Server, socket::default_socket_path};
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

    let min_level = args.level.map(|l| Level::parse(&l));

    let config = hyprdt::server::ServerConfig {
        socket_path,
        app_filter: args.app,
        min_level,
        show_timestamps: !args.no_timestamps,
    };

    let server = Server::with_config(config);

    // Handle Ctrl+C
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    ctrlc_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    });

    if let Err(e) = server.run() {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}

fn ctrlc_handler<F: FnOnce() + Send + 'static>(handler: F) {
    let handler = std::sync::Mutex::new(Some(handler));
    let _ = ctrlc::set_handler(move || {
        if let Ok(mut guard) = handler.lock() {
            if let Some(h) = guard.take() {
                h();
            }
        }
        std::process::exit(0);
    });
}
