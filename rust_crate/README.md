# Rebuilding the OS — Core System Utilities

Reimplementing classic core OS system utilities from scratch in Rust. Built for the **Rebuilding the OS: Core System Utilities Hackathon**.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

## Quick Start

```bash
# Build
cargo build

# Run
cargo run

# Run with verbose output
cargo run -- --verbose
```

## Development

```bash
# Run all tests (unit + integration)
cargo test

# Lint
cargo clippy

# Format
cargo fmt

# Build optimized release binary
cargo build --release
```

## Project Structure

```
src/
├── main.rs    # Entry point and CLI dispatch
├── cli.rs     # CLI argument definitions (clap)
├── lib.rs     # Library root — public modules
├── error.rs   # Custom error types (thiserror)
└── utils.rs   # Shared helper functions
tests/
└── integration_test.rs   # Integration tests (assert_cmd)
```

## License

This project is licensed under the GPL-3.0 License — see the [LICENSE](LICENSE) file for details.