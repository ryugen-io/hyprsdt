//! Client-side tracing layer for sending logs to hyprdt

use crate::socket::{Level, Message, socket_path_for};
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::sync::Mutex;
use tracing::Subscriber;
use tracing_subscriber::Layer;

/// A tracing Layer that sends events to a hyprdt server
pub struct HyprdtLayer {
    app_name: String,
    socket: Mutex<Option<UnixStream>>,
}

impl HyprdtLayer {
    /// Create a new layer for the given app name
    pub fn new(app_name: &str) -> Self {
        let socket_path = socket_path_for(app_name);
        Self::with_socket(app_name, &socket_path)
    }

    /// Create a new layer with a custom socket path
    pub fn with_socket(app_name: &str, socket_path: &Path) -> Self {
        let socket = UnixStream::connect(socket_path).ok();
        Self {
            app_name: app_name.to_string(),
            socket: Mutex::new(socket),
        }
    }

    /// Check if connected to hyprdt server
    pub fn is_connected(&self) -> bool {
        self.socket
            .lock()
            .map(|guard| guard.is_some())
            .unwrap_or(false)
    }

    /// Try to reconnect to the server
    pub fn reconnect(&self) {
        let socket_path = socket_path_for(&self.app_name);
        self.reconnect_to(&socket_path);
    }

    /// Try to reconnect to a specific socket path
    pub fn reconnect_to(&self, socket_path: &Path) {
        if let Ok(mut guard) = self.socket.lock() {
            *guard = UnixStream::connect(socket_path).ok();
        }
    }

    fn send_message(&self, msg: &Message) {
        if let Ok(mut guard) = self.socket.lock() {
            if let Some(stream) = guard.as_mut() {
                let wire = msg.to_wire();
                if stream.write_all(wire.as_bytes()).is_err() {
                    *guard = None;
                }
            }
        }
    }
}

impl<S> Layer<S> for HyprdtLayer
where
    S: Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let metadata = event.metadata();
        let level = Level::from_tracing(metadata.level());

        struct MessageVisitor {
            message: String,
            scope: Option<String>,
        }

        impl tracing::field::Visit for MessageVisitor {
            fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
                if field.name() == "message" {
                    self.message = format!("{:?}", value);
                    if self.message.starts_with('"') && self.message.ends_with('"') {
                        self.message = self.message[1..self.message.len() - 1].to_string();
                    }
                }
            }

            fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
                match field.name() {
                    "message" => self.message = value.to_string(),
                    "scope" => self.scope = Some(value.to_string()),
                    _ => {}
                }
            }
        }

        let mut visitor = MessageVisitor {
            message: String::new(),
            scope: None,
        };
        event.record(&mut visitor);

        if visitor.message.is_empty() {
            return;
        }

        let mut msg = Message::new(&self.app_name, level, &visitor.message);
        if let Some(scope) = visitor.scope {
            msg = msg.with_scope(&scope);
        }

        self.send_message(&msg);
    }
}
