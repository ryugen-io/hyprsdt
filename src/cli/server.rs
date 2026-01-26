//! Server command

use crate::Server;
use crate::server::ServerConfig;
use crate::socket::socket_path_for;
use clap::Parser;

use super::util;

#[derive(Parser)]
pub struct ServerArgs {
    /// App name (determines socket path)
    pub app: String,

    /// Hide timestamps
    #[arg(long)]
    pub no_timestamps: bool,

    /// Compact single-line output
    #[arg(long)]
    pub compact: bool,

    /// JSON output (for piping)
    #[arg(long)]
    pub json: bool,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// Run in current terminal (don't spawn new window)
    #[arg(long)]
    pub here: bool,
}

pub fn run(args: ServerArgs) {
    if !args.here {
        util::spawn_in_terminal(&args);
        return;
    }

    run_server(args);
}

fn run_server(args: ServerArgs) {
    let socket_path = socket_path_for(&args.app);
    let cleanup_path = socket_path.clone();

    let config = ServerConfig {
        socket_path,
        app_name: args.app,
        show_timestamps: !args.no_timestamps,
        compact: args.compact,
        json_output: args.json,
        no_color: args.no_color,
    };

    let server = Server::with_config(config);

    let _ = ctrlc::set_handler(move || {
        let _ = std::fs::remove_file(&cleanup_path);
        std::process::exit(0);
    });

    server.run().expect("[ERROR] Server failed");
}
