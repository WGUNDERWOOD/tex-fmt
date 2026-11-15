# Repository Guidelines

## Project Structure & Module Organization

tex-fmt is a LaTeX formatter written in Rust (edition 2021). `src/bin.rs` provides the CLI entry point; `src/lib.rs` exposes the library. Formatting logic lives in focused modules such as `indent.rs`, `wrap.rs`, `table.rs`, and `comments.rs`; argument and configuration handling live in `cli.rs`, `args.rs`, and `config.rs`.

`src/tests.rs` runs fixtures under `tests/`. Each case contains matching `source/` and `target/` files, optionally accompanied by `tex-fmt.toml` or `cli.txt`. `web/` contains the browser interface, `extra/` holds development scripts and graphics, and `man/`, `completion/`, and `ctan/` support distribution and documentation.

## Build, Test, and Development Commands

- `cargo build --release` (or `just build`): build the optimized executable.
- `cargo run -- --print --noconfig path/to/file.tex`: preview formatting locally.
- `cargo test --no-fail-fast` (or `just test`): run the standard test suite.
- `cargo fmt --check`: check Rust formatting, as CI does; use `cargo fmt` to apply it.
- `cargo clippy --release`: run Rust lint checks. `just clippy` also runs `cargo shear` for unused dependencies.
- `cargo doc`: generate documentation for the internals.
- `just wasm`: build the browser package; requires the WASM target, wasm-bindgen, and wasm-opt.

## Coding Style & Naming Conventions

Use rustfmt with the repository's 80-column maximum and default four-space Rust indentation. Follow existing `snake_case` names for modules, functions, and variables, and `PascalCase` for types. The library enables Clippy's pedantic warnings. Keep formatting behavior in the relevant focused module.

## Testing Guidelines

Use Rust's built-in test framework. Add regression fixtures with descriptive directory names and identical filenames under `source/` and `target/`. The harness checks expected output and idempotence. Supply case-specific configuration when needed. No numeric coverage threshold is configured. CI tests stable Rust on Linux, macOS, and Windows.

## Commit & Pull Request Guidelines

Recent commits use short, direct subjects such as `Fix table regex bug`; no mandatory prefix convention is evident. Keep commits focused. Describe the problem, resulting behavior, and validation in pull requests; link relevant issues and include minimal LaTeX examples for formatting changes. Ensure tests and formatting checks pass.

## Configuration Tips

The repository's `tex-fmt.toml` applies automatically when running inside this checkout. Use `--noconfig` or `--config <PATH>` for controlled experiments.
