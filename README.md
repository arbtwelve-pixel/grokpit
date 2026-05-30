# GROKPIT

**Secure, observable, protocol-native bridge that lets you chat in Grok Desktop / grok.com and reliably drive the full power of local Grok Build agents.**

This is general-purpose infrastructure for rapid agentic prototyping. Stay in the fluid main Grok experience while instantly delegating deep local work (codebase understanding, multi-file edits, shell, long-running tasks, subagents, etc.) to Grok Build — with strong security, full observability, and resumability.

Built on the official ACP + MCP protocols that Grok already provides.

---

## Current Status (Phase 0 in progress)

- [x] Core service skeleton + real MCP server over stdio
- [x] Basic policy engine + safe defaults
- [x] GROKPIT skill for main Grok
- [x] Real ACP wiring — delegate_task now actually spawns and runs a local Grok Build agent
- [ ] Polish + streaming updates + better error handling
- [ ] Excellent documentation and mental model

**Current State (Strong Foundation)**: 
- Real MCP server + real ACP client wiring complete
- `delegate_task` is **non-blocking** (background + polling model)
- You submit work → get a task_id instantly → poll `get_task_status` for results

This is the core loop you wanted: stay in the main Grok chat while powerful local agents do the heavy lifting safely in the background.

---

## Why This Exists

The main Grok (Desktop/web) is excellent for ideation, planning, and high-level thinking.

Grok Build (this local CLI/TUI) is world-class at deep, local, agentic coding work with real tools and long context.

**GROKPIT** removes the friction of manually copying context between them.

It becomes the permanent pit you reach for on every new prototype or debugging session.

---

## Core Principles (Non-Negotiable Foundation)

- **Protocol native**: 100% ACP + MCP. No custom wire formats.
- **Security & policy first**: Configurable, auditable, default-safe. Human approval gates for anything dangerous.
- **Observable & resumable**: Full session history, thoughts, tool traces. Resume or rewind from either surface.
- **General purpose**: Works for any project. Your video production app, future tools, random experiments — all benefit.
- **Minimal magic**: Clear mental model. You always understand what is running locally and why.
- **Long-running but resource-respectful**: Background service when active; truly idle when no work (no spinning, low memory).

---

## High-Level Architecture

```
Main Grok (Desktop / Web)
        │  (via MCP tools + good skill prompting)
        ▼
GROKPIT (long-running service)
  ├─── MCP Server (clean, safe, high-level tools)
  ├─── Policy Engine (TOML rules + hooks + approval gates)
  ├─── ACP Client Manager (drives grok agent stdio instances)
  └─── Task / Session Registry (maps high-level work to local sessions + worktrees)
        │
        ▼
Local Grok Build Agent(s)
  (full tools, subagents, memory, sandbox, etc.)
```

See [docs/architecture.md](./docs/architecture.md) for the detailed design.

---

## Tech Approach (Hybrid as Requested)

- **Rust** for the core service (reliability, ACP/MCP fidelity, small footprint, consistency with Tauri apps).
- **TypeScript** where it gives speed (skill development, quick experiments, future UI pieces).
- Long-running background service with proper idle behavior.

---

## Quick Start (Current State)

1. Build `grokpit-core` and register it as an MCP server (see `docs/register-as-mcp.md`).
2. In normal Grok Desktop, describe the work you want done locally.
3. GROKPIT returns a `task_id` immediately (non-blocking).
4. Poll `grokpit__get_task_status` to watch progress and retrieve results.
5. The local agent runs with full power and your normal approval/safety model.

This is the core "chat in main Grok → powerful local execution" loop.

**Bonus superpower**: Because GROKPIT now supports HTTP mode, you can also control the exact same local agents from the official **Grok app on your iPhone** while you're out running errands. See `docs/remote-ios-control.md`.

---

## Relationship to Other Work

- The ambitious uncensored AI video/photo production desktop app (in `..\GROK BUILD APP 1`) is paused but fully preserved. GROKPIT will make finishing it (and every future prototype) dramatically faster.
- GROKPIT is deliberately **general infrastructure** first. The video app (and all future projects) become early consumers.

---

## Project Layout

```
GROKPIT/
├── README.md
├── docs/
│   ├── architecture.md
│   ├── remote-ios-control.md
│   └── register-as-mcp.md
├── grokpit-core/            # Rust — MCP + ACP + policy engine
├── grokpit-skill/           # SKILL.md + examples for main Grok
├── policies/                # Default + example policy sets
└── ...
```

---

**Built as a rock-solid foundation for agentic prototyping with Grok — 2026**