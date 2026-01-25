//! hyprdt - Debug terminal for the hypr* ecosystem
//!
//! This crate provides:
//! - A Unix socket-based debug log viewer (server)
//! - A tracing Layer for sending logs to hyprdt (client)
//!
//! # Usage
//!
//! ## As a library (client)
//! ```rust,ignore
//! use hyprdt::HyprdtLayer;
//! use tracing_subscriber::prelude::*;
//!
//! // Connect to running hyprdt server
//! let layer = HyprdtLayer::new("myapp");
//! tracing_subscriber::registry().with(layer).init();
//!
//! // Now all tracing events go to hyprdt
//! tracing::info!("Hello from myapp!");
//! ```
//!
//! ## As a server (CLI)
//! ```bash
//! hyprdt              # Start server with default socket
//! hyprdt --app myapp  # Filter to specific app
//! ```

pub mod client;
pub mod server;
pub mod socket;

pub use client::HyprdtLayer;
pub use server::Server;
pub use socket::{Message, default_socket_path};

/// Re-export for convenience
pub use tracing;
