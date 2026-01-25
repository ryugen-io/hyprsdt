//! hyprdt CLI - Debug terminal server

use clap::Parser;
use hyprdt::Server;
use hyprdt::socket::{Level, default_socket_path};
use std::path::PathBuf;
use std::process::{Command, Stdio};

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

    /// Run in current terminal (don't spawn new window)
    #[arg(long)]
    here: bool,
}

fn main() {
    let args = Args::parse();

    // Always spawn new terminal unless --here specified
    if !args.here {
        spawn_in_terminal(&args);
        return;
    }

    run_server(args);
}

fn spawn_in_terminal(args: &Args) {
    let terminal = std::env::var("TERMINAL").ok().or_else(|| {
        let terminals = [
            "rio",
            "alacritty",
            "kitty",
            "foot",
            "gnome-terminal",
            "xterm",
        ];
        for term in terminals {
            if which::which(term).is_ok() {
                return Some(term.to_string());
            }
        }
        None
    });

    let Some(term) = terminal else {
        eprintln!("No terminal found. Run with --here or set $TERMINAL");
        std::process::exit(1);
    };

    // Build args for respawn
    let exe = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("hyprdt"));
    let mut cmd_args = vec!["--here".to_string()];

    if let Some(ref socket) = args.socket {
        cmd_args.push("--socket".to_string());
        cmd_args.push(socket.display().to_string());
    }
    if let Some(ref app) = args.app {
        cmd_args.push("--app".to_string());
        cmd_args.push(app.clone());
    }
    if let Some(ref level) = args.level {
        cmd_args.push("--level".to_string());
        cmd_args.push(level.clone());
    }
    if args.no_timestamps {
        cmd_args.push("--no-timestamps".to_string());
    }

    let _ = Command::new(&term)
        .arg("-e")
        .arg(&exe)
        .args(&cmd_args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}

fn run_server(args: Args) {
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
