//! CLI module for hyprdt

mod send;
mod server;
mod util;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "hyprdt", version, about = "hyprdt")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the debug terminal server
    Server(server::ServerArgs),

    /// Send a test log message
    Send(send::SendArgs),

    /// Check if server is running for an app
    Ping {
        /// App name
        app: String,
    },

    /// Show server status for an app
    Status {
        /// App name
        app: String,
    },
}

impl Cli {
    pub fn run(self) {
        match self.command {
            Commands::Server(args) => {
                server::run(args);
            }
            Commands::Send(args) => {
                send::run(args);
            }
            Commands::Ping { app } => {
                run_ping(&app);
            }
            Commands::Status { app } => {
                run_status(&app);
            }
        }
    }
}

fn run_ping(app: &str) {
    use crate::socket::socket_path_for;
    use std::os::unix::net::UnixStream;

    let socket_path = socket_path_for(app);

    if !socket_path.exists() {
        panic!("[INFO] Server not running for {}", app);
    }

    UnixStream::connect(&socket_path)
        .unwrap_or_else(|_| panic!("[WARN] Socket exists but not responding for {}", app));

    println!("[OK] Server running for {}", app);
}

fn run_status(app: &str) {
    use crate::socket::socket_path_for;
    use std::os::unix::net::UnixStream;

    let socket_path = socket_path_for(app);

    println!("hyprdt status [{}]", app);
    println!("  Socket: {:?}", socket_path);

    if socket_path.exists() {
        match UnixStream::connect(&socket_path) {
            Ok(_) => println!("  Status: running"),
            Err(_) => println!("  Status: socket exists but not responding"),
        }
    } else {
        println!("  Status: not running");
    }
}
