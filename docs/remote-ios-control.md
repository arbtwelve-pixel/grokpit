# Remote Control of GROKPIT from iPhone Grok

This is one of the most powerful use cases for GROKPIT:

You start a heavy coding/debugging session at home using GROKPIT + local Grok Build.
You have to leave (errands, gym, etc.).
You want the Grok app on your iPhone to be able to monitor, steer, and even take full control of the local agent remotely.

## Architecture

```
iPhone Grok App  ──(MCP over HTTPS)──▶  GROKPIT (running at home, exposed securely)
                                              │
                                              ▼
                                    Local Grok Build Agent
```

## Step-by-Step Setup for Remote iOS Control

### 1. Run GROKPIT in HTTP Server Mode

On your home machine:

```bash
# Recommended: with strong auth
./grokpit --http-port 9876 --auth-token "super-secret-long-token-here"

# Or for development (insecure)
./grokpit --http-port 9876
```

### 2. Expose GROKPIT Securely to the Internet

**Best options (ranked):**

| Method              | Security | Difficulty | Recommendation |
|---------------------|----------|------------|----------------|
| **Tailscale**       | Excellent | Easy      | **Best choice** |
| Cloudflare Tunnel   | Very Good | Medium    | Great          |
| ngrok               | Good      | Easy      | Quick & dirty  |
| Port forward + HTTPS| Risky     | Hard      | Avoid          |

**Tailscale example (strongly recommended):**
1. Install Tailscale on your home machine and iPhone.
2. Run GROKPIT with `--http-port 9876`.
3. On iPhone, use the Tailscale IP: `http://100.x.x.x:9876`

No public exposure at all — much safer.

### 3. Register the Remote MCP Server in Your Grok Account

Add this to your `~/.grok/config.toml` (or wherever your main Grok config lives):

```toml
[mcp_servers.grokpit]
url = "http://100.x.x.x:9876/mcp"          # Tailscale IP or Cloudflare URL
headers = { Authorization = "Bearer super-secret-long-token-here" }
enabled = true
```

After adding it, restart your Grok session (or use `/mcps` → reload).

Both your **Desktop Grok** and **iPhone Grok app** will now see the same `grokpit__*` tools.

### 4. Use It From Your iPhone

While away, you can say things like:

- "Check on my GROKPIT task gpk_abc123 and tell me the current status."
- "Tell the local agent in GROKPIT to stop what it's doing and focus on fixing the authentication bug instead."
- "The local build is failing. Use grokpit to investigate and give me a summary on my phone."

The iPhone Grok can now take the lead on long-running local work.

## Security Notes (Important)

- Always use `--auth-token` when exposing over the network.
- Prefer Tailscale or Cloudflare Tunnel over raw port forwarding.
- The local Grok Build agent still respects its normal permission model and will ask for approvals on dangerous actions (this is good).

## Future Enhancements (Planned)

- `grokpit__send_follow_up` — Send new instructions to a running task (already implemented).
- Real-time push notifications when tasks complete or hit approval gates.
- Ability to "pause" or "redirect" the local agent from the phone.

This remote control capability is exactly why GROKPIT exists.