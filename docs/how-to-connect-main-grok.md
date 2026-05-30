# How to Connect Main Grok (Desktop / iPhone / Web) to Your Local GROKPIT

This is the key setup for the original vision: chatting normally with Grok on your phone or desktop while it can delegate real work to your local powerful Grok Build agent via GROKPIT.

## 1. Run GROKPIT in HTTP Server Mode on Your Machine

On the computer that has your local Grok Build / coding environment:

```bash
# Recommended (with auth)
./grokpit --http-port 9876 --auth-token "a-very-long-random-secret-token-here"

# For local testing only
./grokpit --http-port 9876
```

Keep this process running (or run it as a background service later).

## 2. Expose GROKPIT to the Internet (or Your Devices)

The main Grok apps need to reach your machine.

### Best Option: Tailscale (Strongly Recommended)

1. Install Tailscale on your home coding machine and on your phone.
2. Both devices will get magic IPs in the `100.x.x.x` range.
3. Use the Tailscale IP of your home machine (e.g. `http://100.64.12.34:9876/mcp`).

No port forwarding, no public exposure, encrypted, and works even when you're on mobile data.

### Good Options
- **Cloudflare Tunnel** (free, reliable)
- **ngrok** (quick for testing: `ngrok http 9876`)

Avoid raw port forwarding unless you really know what you're doing.

## 3. Register GROKPIT as a Remote MCP Server

Add this to your Grok configuration (the same config used by Grok Desktop, Web, and mobile apps):

```toml
[mcp_servers.grokpit]
url = "http://YOUR-EXPOSED-ADDRESS:9876/mcp"   # Tailscale IP, Cloudflare URL, or ngrok URL
headers = { "Authorization" = "Bearer a-very-long-random-secret-token-here" }
enabled = true
```

**Important notes:**
- Replace `YOUR-EXPOSED-ADDRESS` with your actual reachable address (Tailscale IP is safest).
- The token must match exactly what you used when starting GROKPIT.
- After adding this, restart Grok Desktop or force a reload of MCP servers (look for `/mcps` or the MCP management UI in the app).

## 4. Test the Connection

In any Grok interface (Desktop, iPhone, or grok.com), try:

> "List the tools available from the grokpit MCP server"

You should see tools like:
- `grokpit__delegate_task`
- `grokpit__get_task_status`
- `grokpit__send_follow_up`
- `grokpit__list_active_tasks`

## 5. Start Using It

Example prompts that now work from your phone:

- "Use grokpit to explore my project at C:\\code\\my-app and summarize the architecture."
- "Check GROKPIT task gpk_abc123 and give me the current status."
- "Tell the local agent via grokpit to stop working on the UI and focus on the backend auth bug instead."

## Security Recommendations

- Always use a long, random `--auth-token`.
- Prefer Tailscale or Cloudflare Tunnel.
- The local Grok Build agent still respects its own permission model and will ask for approvals on dangerous actions.

## Troubleshooting

- GROKPIT not showing up? Make sure the HTTP server is running and reachable from the device you're testing on.
- Authentication errors? Double-check the token in both the command line and the config.
- Tools not appearing? Restart the Grok app completely after editing the config.

Once this is set up, you can leave your house and still have the full power of your local coding agent available from your phone.