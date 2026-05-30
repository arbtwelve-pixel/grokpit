# GROKPIT — Architecture

This document captures the approved design for a rock-solid, general-purpose bridge between Grok Desktop/web and local Grok Build agents.

**Core Goal**: Let you stay in the main Grok chat while reliably, safely, and observably driving the full local coding agent.

---

## Guiding Principles

1. **Protocol-native only** — ACP for rich agent control + streaming. MCP for exposing capabilities to the main Grok. No custom protocols.
2. **Security & policy as first-class citizens** — Configurable, auditable, default-deny for dangerous operations. Human approval is easy and visible.
3. **Observable & resumable by default** — Every delegation has full lineage in `~/.grok/sessions/`. You can inspect, resume, or rewind from either surface.
4. **Thin orchestrator** — Conductor coordinates and enforces policy. It does *not* duplicate Grok Build's intelligence.
5. **General purpose** — Works for any codebase or project. Your video app, future tools, experiments — all benefit equally.
6. **Long-running but truly idle** — Background service that wakes for work and consumes almost nothing when idle.

---

## High-Level Components

### 1. GROKPIT Core Service (Rust)
- **MCP Server**: Exposes a small, safe, high-level tool surface to the main Grok (implemented).
  - `grokpit__delegate_task` ← now wired to real ACP
  - `grokpit__get_task_status`
  - `grokpit__list_active_tasks`
- **ACP Client Manager**: Spawns `grok agent stdio`, performs handshake, sends prompt, and collects result (basic wiring complete).
- **Policy Engine**: Basic version in place (more rules coming).
- **Task Registry**: Now tracks real tasks with status and results.
- **Notification Router**: Currently returns final result. Rich streaming is future work.

### 2. GROKPIT Skill (for Main Grok)
- Lives in `~/.grok/skills/grokpit/` (or installed via plugin).
- Teaches the main Grok exactly when and how to delegate.
- Contains excellent examples, argument schemas, and mental model guidance.
- Auto-discovered via normal skill mechanisms.

### 3. Policy & Approval Layer
- Global + per-project TOML policies.
- Explicit approval tool that main Grok must surface to the user.
- Full audit log of every decision.
- Capability modes per delegation (read-only, read-write with limits, execute with allowlist).

### 4. Optional Thin UI (Phase 2+)
- System tray + small status window.
- Shows active delegations, pending approvals, logs.
- One-click approve/deny/cancel.
- Can be a tiny separate Tauri app or embedded.

---

## Data & Isolation Model

- **GROKPIT Task** = the unit main Grok reasons about (high-level ID, description, status, result artifacts).
- Each task owns one or more **Grok Build sessions**.
- Edits happen in **git worktrees** (isolated until explicitly merged or discarded).
- Full history lives in the normal `~/.grok/sessions/` tree → full rewind / compact / share support works automatically.
- Lineage is preserved (which main-Grok prompt led to which GROKPIT task → which Build sessions).

---

## Security Model (Configurable + Safe Defaults)

Default policy (conservative but usable):
- Full read access inside the declared workspace.
- Write access only to paths the task was explicitly scoped to.
- Shell commands go through an allowlist + explicit approval for anything outside the list.
- Git operations (push, force, etc.) always require approval.
- Network access can be gated per task.

All policy decisions are logged with the triggering main-Grok prompt ID for auditability.

The local Grok Build TUI remains the ultimate source of truth for approvals when the user is at the machine.

---

## Communication Patterns

**Main Grok → GROKPIT**
- Via MCP tools (primary).
- The GROKPIT skill gives main Grok the right vocabulary and judgment about when to delegate.

**GROKPIT → Local Grok Build**
- Via official ACP (rich streaming of thoughts, tool calls, plan mode, etc.).
- Managed `grok agent stdio` processes (or on-demand for simpler cases).

**Feedback**
- Synchronous via MCP tool results (status, summaries, final artifacts).
- Rich session artifacts in `~/.grok/sessions/` for deep inspection.
- Future: optional lightweight push notifications.

---

## Why This Architecture Wins

- Leverages every existing Grok primitive (ACP, MCP, sessions, worktrees, subagents, skills, hooks, policy system).
- Clean security boundary between "cloud/main Grok" and "local powerful agent".
- Excellent mental model for the user: "I told GROKPIT to do X. It used local Grok Build under these rules."
- Future-proofs against changes in either surface.

---

## Open Implementation Questions (Phase 0)

- Exact MCP tool surface (keep it very small initially).
- How aggressively to pool `grok agent stdio` processes.
- Idle behavior for the background service (Windows service + smart suspend).
- Packaging story (standalone + Grok plugin).

These will be resolved with working code and real usage in Phase 0.

---

**This architecture was approved after thorough exploration of the Grok Build integration surfaces (ACP, MCP, headless, hooks, skills, subagents, sessions). It is deliberately conservative on invention and aggressive on using what already works extremely well.**