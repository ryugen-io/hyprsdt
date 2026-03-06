//! hyprsdt - Debug terminal for the hypr* ecosystem
//!
//! Each app gets its own socket: `hyprsdt-{app}.sock`
//!
//! # Usage
//!
//! ## As a library (client)
//! ```rust,ignore
//! use hyprsdt::HyprdtLayer;
//! use tracing_subscriber::prelude::*;
//!
//! let layer = HyprdtLayer::new("myapp");
//! tracing_subscriber::registry().with(layer).init();
//!
//! tracing::debug!("Hello from myapp!");
//! ```
//!
//! ## As a server (CLI)
//! ```bash
//! hyprsdt server myapp   # Start server for myapp
//! hyprsdt ping myapp     # Check if running
//! ```

pub mod client;
pub mod server;
pub mod socket;

#[cfg(feature = "cli")]
pub mod cli;

pub use client::HyprdtLayer;
pub use server::Server;
pub use socket::{Message, socket_path_for};

pub use tracing;
