# Registering GROKPIT as an MCP Server

Once the core is built, you can register it with Grok so the main Grok Desktop / web can call it.

## 1. Build the binary (when Rust is available)

```bash
cd grokpit-core
cargo build --release
```

The binary will be at `target/release/grokpit` (or `grokpit.exe` on Windows).

## 2. Add to your Grok config

Edit `~/.grok/config.toml`:

```toml
[mcp_servers.grokpit]
command = "C:\\Users\\ANDREW\\Projects\\GROKPIT\\grokpit-core\\target\\release\\grokpit.exe"
args = []
enabled = true
startup_timeout_sec = 15
```

Or on a typical dev setup:

```toml
[mcp_servers.grokpit]
command = "cargo"
args = ["run", "--manifest-path", "C:\\Users\\ANDREW\\Projects\\GROKPIT\\grokpit-core\\Cargo.toml", "--"]
enabled = true
```

## 3. Restart / reload Grok

- In the TUI: run `/mcps` and reload, or restart the session.
- Use `grok mcp list` to verify it appears.

## 4. Test it

In normal Grok Desktop or grok.com chat, say:

> "List the tools available from the grokpit MCP server"

Or more powerfully:

> "Use grokpit to explore my current project and summarize the structure."

GROKPIT should now be reachable.

## Future Improvements

- Proper packaging (install to a stable path)
- Auto-start background service
- Windows service registration
- Richer progress notifications back to main Grok

This is the foundation for making local agent power feel native from the main Grok interface.