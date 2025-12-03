# Advent of Code

This workspace contains solutions for Advent of Code challenges organized by year.

## Structure

```
advent-of-code/
├── Cargo.toml          # Workspace root
├── year2025/           # 2025 solutions
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       └── day1/
├── year2026/           # 2026 solutions (when added)
├── year2027/           # 2027 solutions (when added)
└── ...
```

## Running Solutions

Run a specific year:
```sh
cargo run -p year2025
```

Run tests for a specific year:
```sh
cargo test -p year2025
```

Check all crates:
```sh
cargo check --workspace
```

## Adding a New Year

1. Create a new directory: `mkdir yearXXXX`
2. Create `yearXXXX/Cargo.toml`:
```toml
[package]
name = "yearXXXX"
version.workspace = true
edition.workspace = true

[dependencies]
```
3. Add the crate to workspace members in root `Cargo.toml`
4. Create `yearXXXX/src/` with your solutions
