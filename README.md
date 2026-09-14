# dsorter

A background tool that watches a folder (like Downloads) and automatically sorts new files into folders based on their type.

## What's coming next

- Run as a background daemon (`launchd` on macOS, `systemd` on Linux)

## Why

Downloads folders turn into a mess fast. This automates the sorting instead of doing it by hand.

## Tech

- Rust
- `notify` for filesystem events
- `ratatui` planned for the TUI

## Status

Early / work in progress.
