# Paperclip desktop

Paperclip desktop is a macOS app that opens a local Paperclip instance in a
native window. It is intended for people who use Codex and Claude Code through
their existing subscriptions.

The desktop shell is custom to this repository. The dashboard, onboarding,
projects, tasks, agents, and adapters are upstream Paperclip features.

## What you need

- macOS
- Node.js 24.11 or newer (the desktop service is the supported Node CLI)
- An authenticated Codex CLI or Claude Code CLI subscription
- The Paperclip desktop app bundle, or this repository if you are building it

This MVP does not enable OpenAI or Anthropic API key billing. Do not add
provider API keys to Paperclip configuration for the desktop instance.

## First launch

1. Open `Paperclip.app`. On a new Mac user profile, it finds the managed
   Agentic Paperclip CLI or installs it from this fork using `npx`. Keep the
   window open while this first setup completes.
2. Enter an organization name that can cover all of your work, such as your
   name, studio, or company. Projects are created inside that organization.
3. Complete the Paperclip onboarding flow.
4. Create a project, connect its local repository, and create a task.
5. Configure an agent to use the Codex or Claude Code local adapter. Sign in
   to the matching CLI on your Mac when Paperclip asks for access.

Closing the Paperclip window does not stop the local service. macOS sleep or
shutdown pauses local work.

## Everyday use

Use **Projects** to keep repositories separate and **Tasks** to describe work.
The initial Dori interview is an upstream Paperclip option for turning an idea
into a plan. Choose **I have a task in mind** when you already know what you
want to do.

The local desktop instance stores its state under:

```text
~/.paperclip/instances/desktop
```

This includes Paperclip data and generated artifacts. Your subscription
credentials stay with the official Codex and Claude Code CLIs and are not
copied into Paperclip.

## If Paperclip does not open

Paperclip shows a local recovery screen instead of an empty window when the
CLI or service cannot start. It includes the exact failure reason. Install the
current Node.js LTS if Node 24.11 or newer is absent, then run:

```sh
npx --yes --package paperclipai@latest paperclipai install --yes \
  --repo carherpi/paperclip --ref master
```

The app only uses the managed CLI built from `carherpi/paperclip`, which carries
the desktop subscription-only policy; it does not use a checkout-local
development shim or an arbitrary global `paperclipai` install. These commands
show and control the background service:

```sh
paperclipai service status --instance desktop
paperclipai service logs --instance desktop
paperclipai service stop --instance desktop
paperclipai service start --instance desktop
```

If a service was interrupted, start it with the final command and reopen
`Paperclip.app`. Do not use `paperclipai run` for this instance: the managed
LaunchAgent prevents two server processes from writing the same desktop data.
Opening and closing `Paperclip.app` reconciles the service definition but does
not restart a healthy service, so active work continues across window closes
and reopens.

## Backing up or moving to another Mac

1. Stop the desktop service.
2. Copy `~/.paperclip/instances/desktop` and any unpushed repository changes.
3. On the new Mac, install Paperclip desktop plus the Codex and Claude Code
   CLIs, then authenticate those CLIs again.
4. Restore the files, confirm local repository paths and existing pull
   requests, and only then restart the service.

Do not run the old and restored services at the same time.

## Current MVP limits

- Telegram questions and approvals are not configured.
- Telegram credentials are not stored through the macOS Keychain.
- The custom approval, review, repair, and pull request workflow is not yet
  implemented.
- Disposable isolated execution containers are not yet included.

## Building from source

Install the repository dependencies, then the pinned desktop dependencies:

```sh
pnpm install
pnpm --dir desktop --ignore-workspace install
pnpm --dir desktop --ignore-workspace build
```

For source development only, `PAPERCLIP_DESKTOP_CLI` can point to an explicit
CLI binary. Release builds locate a normal installed CLI instead. The build
requires the Xcode Command Line Tools, Rust, and Node.js 24.11 or newer; when a
runtime prerequisite is missing at launch, the app shows its recovery screen.

The built app bundle is created under
`desktop/src-tauri/target/release/bundle/macos/`.
