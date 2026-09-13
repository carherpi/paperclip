# Paperclip desktop subscription MVP

The macOS desktop host is a Tauri 2 shell for a local Paperclip service. It
uses the existing `paperclipai service` LaunchAgent rather than starting a
second server from the app window. Closing the window therefore leaves active
work running until macOS sleeps or shuts down.

The host creates a separate `desktop` instance with
`PAPERCLIP_SUBSCRIPTION_ONLY=1`. Its LaunchAgent contains no provider key or
token. Sign in through the official `codex` and `claude` CLIs on the Mac; do
not copy those credentials into Paperclip configuration.

## Development

Install the root workspace dependencies, then install the pinned desktop
package dependencies and run:

```sh
pnpm --dir desktop install
pnpm --dir desktop dev
```

The host invokes `paperclipai service install --instance desktop`. Set
`PAPERCLIP_DESKTOP_CLI` when testing an unpublished CLI binary. The service
uses the normal Paperclip data location under its `desktop` instance, so tasks,
sessions, questions, and artifacts survive desktop restarts.

## Backup and moving Macs

1. Stop the desktop instance with `paperclipai service stop --instance desktop`.
2. Copy the instance data directory and any unpushed execution workspaces.
3. On the destination Mac, install Paperclip and the Codex and Claude CLIs,
   then sign in again. Credentials are intentionally not part of the backup.
4. Restore the data, inspect repository paths and existing pull requests, test
   Telegram delivery if configured, and resume work only after those checks.
5. Do not start the destination service until the old runner is stopped.
