//! GROKPIT Core Service
//!
//! Secure, observable bridge that exposes a clean MCP surface to the main Grok
//! while driving full-powered local Grok Build agents via the official ACP protocol.

use std::path::PathBuf;

use clap::Parser;
use tracing::{info, warn};

use grokpit_core::mcp::McpServer;
use grokpit_core::policy::PolicyEngine;
use grokpit_core::task::TaskRegistry;

#[derive(Parser, Debug)]
#[command(name = "grokpit", version, about = "GROKPIT — Grok Desktop ↔ Grok Build Bridge")]
struct Args {
    /// Path to policy file (default: uses built-in safe defaults)
    #[arg(short, long)]
    policy: Option<PathBuf>,

    /// Run as HTTP server on this port (enables remote access from iPhone Grok)
    #[arg(long)]
    http_port: Option<u16>,

    /// Auth token required for remote HTTP access (strongly recommended)
    #[arg(long)]
    auth_token: Option<String>,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    // Initialize tracing
    let level = if args.debug { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!("grokpit_core={},grokpit=info", level))
        .init();

    info!("Starting GROKPIT Core Service v0.1.0");

    // Load policy
    let policy = if let Some(path) = &args.policy {
        info!("Loading policy from: {}", path.display());
        PolicyEngine::load_from_file(path)
    } else {
        info!("Using default safe policy");
        PolicyEngine::default_policy()
    };

    let tasks = TaskRegistry::new();

    if let Some(port) = args.http_port {
        // === HTTP Mode (for remote iPhone / anywhere access) ===
        info!("Starting in HTTP server mode on port {}", port);

        if args.auth_token.is_none() {
            warn!("WARNING: Running HTTP mode without --auth-token is insecure!");
        }

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            grokpit_core::http_server::start_http_server(
                port,
                args.auth_token,
                policy,
                tasks,
            )
            .await;
        });
    } else {
        // === Stdio Mode (local only, for Desktop Grok) ===
        let server = McpServer::new(policy, tasks);

        info!("GROKPIT MCP server ready (stdio mode). Register this binary locally in Grok config.");

        if let Err(e) = server.run_stdio() {
            tracing::error!("MCP server exited with error: {}", e);
            std::process::exit(1);
        }
    }
}