//! Socket utilities and message format

use std::env;
use std::path::PathBuf;

/// Socket path for a specific app
pub fn socket_path_for(app: &str) -> PathBuf {
    let runtime_dir = env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::temp_dir());
    runtime_dir.join(format!("hyprdt-{}.sock", app))
}

/// Message format sent over the socket
#[derive(Debug, Clone)]
pub struct Message {
    pub app: String,
    pub level: Level,
    pub scope: Option<String>,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Local>,
}

impl Message {
    pub fn new(app: &str, level: Level, message: &str) -> Self {
        Self {
            app: app.to_string(),
            level,
            scope: None,
            message: message.to_string(),
            timestamp: chrono::Local::now(),
        }
    }

    pub fn with_scope(mut self, scope: &str) -> Self {
        self.scope = Some(scope.to_string());
        self
    }

    /// Serialize to wire format
    pub fn to_wire(&self) -> String {
        let scope_part = self
            .scope
            .as_ref()
            .map(|s| format!("[{}]", s))
            .unwrap_or_default();
        format!(
            "{}|{}|{}|{}|{}\n",
            self.timestamp.format("%H:%M:%S%.3f"),
            self.app,
            self.level.as_str(),
            scope_part,
            self.message
        )
    }

    /// Parse from wire format
    pub fn from_wire(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.splitn(5, '|').collect();
        if parts.len() < 5 {
            return None;
        }

        let timestamp = chrono::Local::now();
        let app = parts[1].to_string();
        let level = Level::parse(parts[2]);
        let scope = if parts[3].is_empty() {
            None
        } else {
            Some(parts[3].trim_matches(['[', ']']).to_string())
        };
        let message = parts[4].trim_end().to_string();

        Some(Self {
            app,
            level,
            scope,
            message,
            timestamp,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Error => "ERROR",
            Level::Warn => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "ERROR" => Level::Error,
            "WARN" | "WARNING" => Level::Warn,
            "INFO" => Level::Info,
            "DEBUG" => Level::Debug,
            "TRACE" => Level::Trace,
            _ => Level::Info,
        }
    }

    pub fn from_tracing(level: &tracing::Level) -> Self {
        match *level {
            tracing::Level::ERROR => Level::Error,
            tracing::Level::WARN => Level::Warn,
            tracing::Level::INFO => Level::Info,
            tracing::Level::DEBUG => Level::Debug,
            tracing::Level::TRACE => Level::Trace,
        }
    }
}
