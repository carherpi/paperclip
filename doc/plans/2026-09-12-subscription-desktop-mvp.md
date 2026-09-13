# Subscription-only desktop MVP

## Goal

Package Paperclip's existing local control plane as a macOS Tauri 2 host for
operators who run Codex and Claude Code through their existing subscriptions.
The desktop host must keep the local server alive after its window closes and
must not introduce an API-key or paid-provider fallback.

## Existing capabilities reused

- The `codex_local` and `claude_local` adapters already use the official local
  runtimes, persist sessions, report provider quota, and classify quota errors.
- The server already persists task state, interactions, workspaces, recovery
  state, artifacts, and Telegram delivery data in its local instance store.
- Existing quota recovery either schedules a provider-reported reset or keeps
  the task blocked for a board retry.

## Implementation

1. Add a standalone Tauri 2 host in `desktop/` that points at the local
   Paperclip service and exposes only safe local navigation.
   Verify: `cargo check` and the desktop package typecheck pass.
2. Add a launchd installer and service launcher for a user-owned background
   Paperclip process. The service uses a dedicated desktop data root and sets
   `PAPERCLIP_SUBSCRIPTION_ONLY=1`.
   Verify: unit tests cover plist rendering and service command construction.
3. Make the server reject non-subscription executions while that desktop flag
   is set. Keep the upstream default unchanged.
   Verify: focused server tests prove subscription jobs are retained and API
   billing modes are rejected.
4. Document backup and Mac migration: stop the service, copy the desktop data
   root and unpushed workspaces, reconnect credentials on the destination,
   and resume only after validation.

## Deferred

Telegram credential storage through the macOS Keychain, new workflow screens,
and an end-to-end live Codex/Claude run require a signed desktop build and the
operator's authenticated subscriptions. They remain follow-up work after this
foundation is buildable.
