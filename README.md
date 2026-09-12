# citrus

A background tool that watches a folder (like Downloads) and automatically sorts new files into folders based on their type.

## What's coming next

- Actually move files into type-based folders
- Skip files that are still downloading (like `.crdownload` / `.part`)
- Run as a background daemon (`launchd` on macOS, `systemd` on Linux)
- A TUI (built with `ratatui`) to add/remove watched folders and see live activity

## Why

Downloads folders turn into a mess fast. This automates the sorting instead of doing it by hand.

## Tech

- Rust
- `notify` for filesystem events
- `ratatui` planned for the TUI

## Status

Early / work in progress. Not usable yet - just the file-watching and classification pieces so far.