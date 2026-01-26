//! Send command - send test logs to server

use crate::socket::{Level, Message, socket_path_for};
use clap::Parser;
use std::io::Write;
use std::os::unix::net::UnixStream;

#[derive(Parser)]
pub struct SendArgs {
    /// App name (target socket)
    pub app: String,

    /// Log level (error, warn, info, debug, trace)
    pub level: String,

    /// Log message
    pub message: String,

    /// Optional scope
    #[arg(long)]
    pub scope: Option<String>,
}

pub fn run(args: SendArgs) {
    let socket_path = socket_path_for(&args.app);
    let level = Level::parse(&args.level);
    let mut msg = Message::new(&args.app, level, &args.message);

    if let Some(s) = args.scope {
        msg = msg.with_scope(&s);
    }

    let mut stream = UnixStream::connect(&socket_path)
        .unwrap_or_else(|_| panic!("[ERROR] Server not running for {}", args.app));

    let wire = msg.to_wire();
    stream
        .write_all(wire.as_bytes())
        .expect("[ERROR] Failed to send message");

    println!("[OK] Message sent to {}", args.app);
}
