//! Server-side socket listener and log display

use crate::socket::{Level, Message};
use colored::{ColoredString, Colorize};
use std::fs;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Failed to bind socket: {0}")]
    BindError(#[from] std::io::Error),
}

#[derive(Clone)]
pub struct ServerConfig {
    pub socket_path: PathBuf,
    pub app_name: String,
    pub show_timestamps: bool,
    pub compact: bool,
    pub json_output: bool,
    pub no_color: bool,
}

pub struct Server {
    config: ServerConfig,
    running: Arc<AtomicBool>,
}

impl Server {
    pub fn with_config(config: ServerConfig) -> Self {
        Self {
            config,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn run(&self) -> Result<(), ServerError> {
        let socket_path = &self.config.socket_path;

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
                    std::thread::spawn(move || handle_client(stream, &config));
                }
                Err(e) => {
                    eprintln!("{} Accept error: {}", "[ERROR]".red(), e);
                }
            }
        }

        if socket_path.exists() {
            let _ = fs::remove_file(socket_path);
        }

        Ok(())
    }

    fn print_header(&self) {
        if self.config.json_output {
            return;
        }

        if self.config.no_color {
            println!("hyprsdt [{}]", self.config.app_name);
        } else {
            println!(
                "{} [{}]",
                "hyprsdt".bold().underline(),
                self.config.app_name.cyan()
            );
        }

        println!();
    }
}

fn handle_client(stream: UnixStream, config: &ServerConfig) {
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let Ok(line) = line else { break };

        let Some(msg) = Message::from_wire(&line) else {
            println!("{}", line);
            continue;
        };

        print_message(&msg, config);
    }
}

fn print_message(msg: &Message, config: &ServerConfig) {
    if config.json_output {
        print_json(msg);
        return;
    }

    if config.compact {
        print_compact(msg, config);
    } else {
        print_normal(msg, config);
    }
}

fn print_json(msg: &Message) {
    let scope = msg.scope.as_deref().unwrap_or("");
    println!(
        r#"{{"ts":"{}","level":"{}","scope":"{}","msg":"{}"}}"#,
        msg.timestamp.format("%H:%M:%S%.3f"),
        msg.level.as_str(),
        scope,
        msg.message.replace('\\', "\\\\").replace('"', "\\\"")
    );
}

fn print_compact(msg: &Message, config: &ServerConfig) {
    let level = level_char(msg.level);
    let scope = msg
        .scope
        .as_ref()
        .map(|s| format!(":{}", s))
        .unwrap_or_default();

    if config.no_color {
        println!("{}{} {}", level, scope, msg.message);
    } else {
        println!(
            "{}{} {}",
            level_char_colored(msg.level),
            scope.dimmed(),
            msg.message
        );
    }
}

fn print_normal(msg: &Message, config: &ServerConfig) {
    let timestamp = if config.show_timestamps {
        format!("{} ", msg.timestamp.format("%H:%M:%S"))
    } else {
        String::new()
    };

    let scope = msg
        .scope
        .as_ref()
        .map(|s| format!("[{}] ", s))
        .unwrap_or_default();

    if config.no_color {
        println!(
            "{}[{}] {}{}",
            timestamp,
            msg.level.as_str(),
            scope,
            msg.message
        );
    } else {
        println!(
            "{}[{}] {}{}",
            timestamp.dimmed(),
            level_colored(msg.level),
            scope,
            msg.message
        );
    }
}

fn level_char(level: Level) -> char {
    match level {
        Level::Error => 'E',
        Level::Warn => 'W',
        Level::Info => 'I',
        Level::Debug => 'D',
        Level::Trace => 'T',
    }
}

fn level_char_colored(level: Level) -> ColoredString {
    match level {
        Level::Error => "E".red().bold(),
        Level::Warn => "W".yellow(),
        Level::Info => "I".green(),
        Level::Debug => "D".blue(),
        Level::Trace => "T".magenta(),
    }
}

fn level_colored(level: Level) -> ColoredString {
    match level {
        Level::Error => "ERROR".red().bold(),
        Level::Warn => "WARN".yellow(),
        Level::Info => "INFO".green(),
        Level::Debug => "DEBUG".blue(),
        Level::Trace => "TRACE".magenta(),
    }
}
