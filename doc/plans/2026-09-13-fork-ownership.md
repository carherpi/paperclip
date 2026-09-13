# Fork ownership setup

## Goal

Make `carherpi/paperclip` the single repository for the customized Paperclip
desktop MVP while retaining `paperclipai/paperclip` as the update source.

## Steps

1. Create or confirm the `carherpi/paperclip` fork.
   Verify: it has the Paperclip history and accepts the existing desktop MVP
   commit.
2. Set `origin` to the fork and add the official Paperclip repository as
   `upstream`.
   Verify: `git remote -v` lists both destinations correctly.
3. Push the desktop MVP commit to the fork's default branch.
   Verify: the remote branch contains the commit.

## Scope

`carherpi/agentic` remains separate and is not made a submodule owner during
this MVP.
