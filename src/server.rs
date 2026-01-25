//! Server-side socket listener and log display

use crate::socket::{Level, Message, default_socket_path};
use colored::Colorize;
use std::fs;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use thiserror::Error;

/// Server errors
#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Failed to bind socket: {0}")]
    BindError(#[from] std::io::Error),

    #[error("Socket path already in use")]
    SocketInUse,
}

/// Configuration for the server
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Socket path
    pub socket_path: PathBuf,
    /// Filter to specific app (None = all apps)
    pub app_filter: Option<String>,
    /// Minimum log level to display
    pub min_level: Option<Level>,
    /// Show timestamps
    pub show_timestamps: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            socket_path: default_socket_path(),
            app_filter: None,
            min_level: None,
            show_timestamps: true,
        }
    }
}

/// The hyprdt server
pub struct Server {
    config: ServerConfig,
    running: Arc<AtomicBool>,
}

impl Server {
    /// Create a new server with default config
    pub fn new() -> Self {
        Self::with_config(ServerConfig::default())
    }

    /// Create a server with custom config
    pub fn with_config(config: ServerConfig) -> Self {
        Self {
            config,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Check if server is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Stop the server
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Run the server (blocking)
    pub fn run(&self) -> Result<(), ServerError> {
        let socket_path = &self.config.socket_path;

        // Clean up old socket if exists
        if socket_path.exists() {
            fs::remove_file(socket_path)?;
        }

        let listener = UnixListener::bind(socket_path)?;
        self.running.store(true, Ordering::SeqCst);

        self.print_header();

        for stream in listener.incoming() {
            if !self.running.load(Ordering::SeqCst) {
                break;
            }

            match stream {
                Ok(stream) => {
                    let config = self.config.clone();
                    std::thread::spawn(move || {
                        handle_client(stream, &config);
                    });
                }
                Err(e) => {
                    eprintln!("{} Accept error: {}", "[ERROR]".red(), e);
                }
            }
        }

        // Cleanup
        if socket_path.exists() {
            let _ = fs::remove_file(socket_path);
        }

        Ok(())
    }

    fn print_header(&self) {
        println!("{}", "hyprdt - Debug Terminal".bold().underline());
        println!("Socket: {:?}", self.config.socket_path);
        if let Some(ref app) = self.config.app_filter {
            println!("Filter: {}", app.cyan());
        }
        println!();
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

fn handle_client(stream: UnixStream, config: &ServerConfig) {
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let Ok(line) = line else {
            break;
        };

        let Some(msg) = Message::from_wire(&line) else {
            // Fallback: print raw line
            println!("{}", line);
            continue;
        };

        // Apply filters
        if let Some(ref app_filter) = config.app_filter {
            if msg.app != *app_filter {
                continue;
            }
        }

        if let Some(min_level) = config.min_level {
            if !should_show_level(msg.level, min_level) {
                continue;
            }
        }

        print_message(&msg, config);
    }
}

fn should_show_level(level: Level, min: Level) -> bool {
    let level_val = level_to_int(level);
    let min_val = level_to_int(min);
    level_val <= min_val
}

fn level_to_int(level: Level) -> u8 {
    match level {
        Level::Error => 0,
        Level::Warn => 1,
        Level::Info => 2,
        Level::Debug => 3,
        Level::Trace => 4,
    }
}

fn print_message(msg: &Message, config: &ServerConfig) {
    let level_str = match msg.level {
        Level::Error => "ERROR".red().bold(),
        Level::Warn => "WARN".yellow(),
        Level::Info => "INFO".green(),
        Level::Debug => "DEBUG".blue(),
        Level::Trace => "TRACE".magenta(),
    };

    let timestamp = if config.show_timestamps {
        format!("{} ", msg.timestamp.format("%H:%M:%S").to_string().dimmed())
    } else {
        String::new()
    };

    let app = format!("[{}]", msg.app).cyan();

    let scope = msg
        .scope
        .as_ref()
        .map(|s| format!("[{}] ", s))
        .unwrap_or_default();

    println!(
        "{}{}[{}] {}{}",
        timestamp, app, level_str, scope, msg.message
    );
}
