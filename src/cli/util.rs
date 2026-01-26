//! CLI utilities

use std::path::PathBuf;
use std::process::{Command, Stdio};

use super::server::ServerArgs;

/// Spawn hyprdt in a new terminal window
#[allow(clippy::zombie_processes)] // Intentional: parent exits, terminal gets reparented to init
pub fn spawn_in_terminal(args: &ServerArgs) {
    let term =
        find_terminal().expect("[ERROR] No terminal found. Run with --here or set $TERMINAL");

    let exe = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("hyprdt"));
    let cmd_args = build_respawn_args(args);

    Command::new(&term)
        .arg("-e")
        .arg(&exe)
        .args(&cmd_args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("[ERROR] Failed to spawn terminal");
}

fn find_terminal() -> Option<String> {
    std::env::var("TERMINAL").ok().or_else(|| {
        const TERMINALS: &[&str] = &[
            "rio",
            "alacritty",
            "kitty",
            "foot",
            "gnome-terminal",
            "xterm",
        ];

        for term in TERMINALS {
            if which::which(term).is_ok() {
                return Some((*term).to_string());
            }
        }
        None
    })
}

fn build_respawn_args(args: &ServerArgs) -> Vec<String> {
    let mut cmd_args = vec!["server".to_string(), args.app.clone(), "--here".to_string()];

    if args.no_timestamps {
        cmd_args.push("--no-timestamps".to_string());
    }
    if args.compact {
        cmd_args.push("--compact".to_string());
    }
    if args.json {
        cmd_args.push("--json".to_string());
    }
    if args.no_color {
        cmd_args.push("--no-color".to_string());
    }

    cmd_args
}
