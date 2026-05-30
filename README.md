# GROKPIT

**Secure, observable, protocol-native bridge that lets you chat in Grok Desktop / grok.com and reliably drive the full power of local Grok Build agents.**

This is general-purpose infrastructure for rapid agentic prototyping. Stay in the fluid main Grok experience while instantly delegating deep local work (codebase understanding, multi-file edits, shell, long-running tasks, subagents, etc.) to Grok Build — with strong security, full observability, and resumability.

Built on the official ACP + MCP protocols that Grok already provides.

---

## The Killer Feature: Remote Control from iPhone / Desktop Grok

The main reason GROKPIT exists:

You can be out running errands, open the normal Grok app on your phone, and say:

> "Use grokpit to keep working on the refactoring in my home project and update me on progress."

GROKPIT lets the main Grok interfaces delegate real work to your local powerful Grok Build agent in the background.

**See the full setup guide here:** [How to Connect Main Grok to Your Local GROKPIT](docs/how-to-connect-main-grok.md)

---

## Current Status (Phase 0 in progress)

- [x] Core service skeleton + real MCP server over stdio
- [x] Basic policy engine + safe defaults
- [x] GROKPIT skill for main Grok
- [x] Real ACP wiring — delegate_task now actually spawns and runs a local Grok Build agent
- [x] HTTP server mode for remote access
- [ ] Rich streaming updates + better remote experience
- [ ] Packaging and easy installation story

---

## Core Principles

- **Protocol native**: 100% ACP + MCP. No custom wire formats.
- **Security & policy first**: Configurable, auditable, default-safe. Human approval gates for anything dangerous.
- **Observable & resumable**: Full session history, thoughts, tool traces.
- **General purpose**: Works for any project.
- **Long-running but resource-respectful**: Background service that goes quiet when idle.

---

## Quick Links

- [Remote Control Setup (iPhone/Desktop Grok)](docs/how-to-connect-main-grok.md)
- [Architecture](docs/architecture.md)
- [Registering as MCP Server](docs/register-as-mcp.md)

---

**Built as a rock-solid foundation for agentic prototyping with Grok — 2026**