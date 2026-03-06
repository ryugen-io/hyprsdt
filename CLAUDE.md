# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

hyprsdt (hypr debug terminal) is a real-time log viewer for the hypr* ecosystem. It provides:
- A Unix socket-based server that receives and displays logs
- A tracing Layer for Rust applications to send logs to hyprsdt

## Build Commands

```bash
just build          # Build release binary
just test           # Run tests
just lint           # Clippy
just pre-commit     # Full pre-commit checks
just run            # Start server
just run-filter APP # Start server with app filter
```

## Architecture

### Single Crate Structure
```
hyprsdt/
├── src/
│   ├── lib.rs          # Library root, re-exports
│   ├── socket.rs       # Message format, socket path
│   ├── client.rs       # HyprdtLayer (tracing Layer)
│   ├── server.rs       # Server implementation
│   └── bin/
│       └── hyprsdt.rs   # CLI binary
```

### Features
- `default = ["cli"]` - CLI binary enabled by default
- `cli` - Enables clap and ctrlc

### Key Types
- `HyprdtLayer`: tracing Layer for sending logs to server
- `Server`: Unix socket server that displays logs
- `Message`: Log message format
- `Level`: Log level enum

## Protocol

Simple text-based wire format over Unix socket:
```
TIMESTAMP|APP|LEVEL|[SCOPE]|MESSAGE\n
```

Example:
```
14:30:45.123|hyprsink|INFO|[APPLY]|applying template
```

## Usage

### Server (CLI)
```bash
hyprsdt                    # Start with default socket
hyprsdt --app hyprsink      # Filter to hyprsink only
hyprsdt --level warn       # Only warn and error
hyprsdt --no-timestamps    # Hide timestamps
```

### Client (Library)
```rust
use hyprsdt::HyprdtLayer;
use tracing_subscriber::prelude::*;

let layer = HyprdtLayer::new("myapp");
tracing_subscriber::registry().with(layer).init();

tracing::info!(scope = "INIT", message = "Starting up");
```
