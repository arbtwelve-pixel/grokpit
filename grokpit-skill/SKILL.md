---
name: grokpit
description: Use GROKPIT (the local bridge) to delegate deep coding, exploration, scaffolding, debugging, or any task that requires the full power of a local Grok Build agent (file system access, shell commands, long-running work, subagents). Always prefer this when the work involves the user's actual local files or projects rather than pure conversation or cloud-only generation.
when-to-use: "The user wants to work with real local codebases, run commands, edit files on disk, scaffold projects, debug builds, perform multi-step implementation, or any work that benefits from the local agent's full tool access and persistence. Trigger phrases include 'use the local agent', 'via conductor', 'local grok build', 'on my machine', 'in my project folder', etc."
---

# GROKPIT Skill

You have access to **GROKPIT**, a secure local bridge that lets you delegate work to a full-powered Grok Build agent running on the user's machine.

## Mental Model

- You (main Grok) are excellent at high-level thinking, planning, and conversation.
- The local Grok Build agent (via GROKPIT) is excellent at deep codebase work, running real commands, making precise edits, and long agentic loops.
- GROKPIT sits in the middle as a **trusted, policy-aware orchestrator**. It will not let dangerous things happen without user approval.

When the user's request involves their actual local files, projects, or machine state, you should strongly consider delegating via GROKPIT instead of trying to do everything yourself.

## How to Delegate

When GROKPIT is registered as an MCP server, the following tools become available:

- `grokpit__delegate_task` — Submit work (non-blocking — returns immediately with a task_id)
- `grokpit__get_task_status` — Poll for progress and results (call this repeatedly)
- `grokpit__list_active_tasks` — See everything currently running
- `grokpit__send_follow_up` — Send new instructions to an already-running task (great for taking control from your phone)

**Important mental model**: `delegate_task` no longer blocks. It starts the work in the background and gives you a task_id. You then use `get_task_status` to check on it. This keeps the main Grok conversation fluid.

1. **Start a task** with a clear, self-contained description.
2. Provide the target directory or project when relevant.
3. Include any constraints or success criteria.
4. The Conductor will return a task ID and status.
5. Poll for updates or ask the user to check status in the GROKPIT UI / local TUI.
6. For sensitive actions, the user will be asked for explicit approval (either in the local Grok Build TUI or via GROKPIT's surface).

## Good Delegation Patterns

**Excellent use cases:**
- "Explore this codebase and give me a clear architecture map + the 10 most important files."
- "Scaffold a new Tauri + React app with Tailwind and shadcn in C:\Users\ANDREW\Projects\my-new-tool following modern 2026 patterns."
- "Debug why the build is failing in my current project and propose the minimal fix."
- "Implement the user authentication flow we just designed, including tests."
- "Review all recent changes in this repo for security issues and code quality."

**When NOT to delegate:**
- Pure creative writing, general knowledge questions, or planning that doesn't require local files.
- One-off small code snippets the user can copy-paste.
- Anything the user explicitly wants to keep cloud-only.

## Example Prompt Phrasing (for yourself)

When you decide to delegate:

```
Use GROKPIT to [clear task description].

Target directory: [absolute path if known]
Constraints:
- [e.g., only touch files under src/]
- [e.g., must pass existing tests]
- [e.g., follow the style in AGENTS.md]

Success criteria: [what "done" looks like]
```

Then call the appropriate GROKPIT tool with that information.

## Security & User Trust

GROKPIT has a strong policy engine. By default it is conservative:
- Writes are scoped to the declared task/workspace.
- Dangerous shell commands require explicit user approval.
- All actions are fully logged and resumable.

You should be transparent with the user: "I'm going to delegate this to GROKPIT so it can work directly with your files. It will ask for approval on anything sensitive."

## Current Limitations (Phase 0)

- ACP wiring is functional but basic (synchronous delegation + result collection).
- Rich streaming of thoughts/tool calls is not yet forwarded back to you.
- The local Grok Build agent runs with its normal permission model (it may still ask the user for approvals on dangerous actions).

Always confirm with the user before the first delegation in a new project that GROKPIT is set up on their machine.

## Success Criteria for a Good Delegation

- The local agent receives clear scope and goals.
- The user sees visible progress (thoughts, actions, results) without being overwhelmed.
- Sensitive operations are explicitly approved.
- The final result is high-quality and the user can resume the underlying session locally if needed.

Use this skill whenever the user wants real local power while staying in a natural conversation with you.

## Example Flow (What Good Usage Looks Like) — Async Version

User: "Scaffold a new Rust CLI tool called 'filewatcher' in C:\Users\ANDREW\Projects\filewatcher with clap and tokio."

You:
1. Call `grokpit__delegate_task`
   - description: "Scaffold a new Rust CLI tool called filewatcher using clap + tokio..."
   - workspace: "C:\Users\ANDREW\Projects\filewatcher"
   - use_worktree: true   (recommended when making file changes)

2. Immediately tell the user:
   "I've submitted this as task `gpk_abc123` to GROKPIT. It's now running in the background on your machine.
   I'll check the status for you."

3. In subsequent turns, call `grokpit__get_task_status` with `task_id: "gpk_abc123"` to see progress or the final result.

This keeps the conversation responsive even for long-running local work.

**Remote Control from iPhone Example:**

You're out running errands and remember something important:

"Check my current GROKPIT task and tell the local agent to stop working on the UI and instead prioritize fixing the backend auth flow. Then give me a quick status update on my phone."

→ You would use `grokpit__send_follow_up` + `grokpit__get_task_status`.