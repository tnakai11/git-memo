# AGENTS

This repository contains a small Rust CLI. Automated agents and contributors should
follow the rules below when making changes.

## Required checks
- Run `cargo fmt -- --check` and ensure it succeeds.
- Run `cargo clippy -- -D warnings` and fix any lints.
- Run `cargo test --verbose` and ensure all tests pass.

## Commit style
- Keep the summary line under 72 characters.
- Use prefixes like `feat:`, `fix:`, `docs:`, or `chore:` where appropriate.

## Pull requests
- Include a concise description of what was changed and why.
