# Contributing to fsk

Thank you for your interest in contributing to **fsk**. To maintain the high standards of this project, we ask that all contributors adhere to the following guidelines.

## Philosophy

**fsk** is built on three core principles:

1. **Minimalism**: Features must provide high value without unnecessary complexity.
2. **Performance**: Code should be memory-efficient and avoid unnecessary I/O or heap allocations.
3. **Stability**: The master branch must remain deployable at all times.

## Code of Conduct

By participating in this project, you agree to maintain a professional environment. Focus on technical merit and respect for all contributors.

## Development Workflow

### 1. Environment Setup

Ensure you are using the latest stable Rust toolchain.

```bash
rustup update stable
cargo build

```

### 2. Branching Strategy

* **Main**: Production-ready code. No direct commits allowed.
* **Features/Fixes**: Create a descriptive branch from `main` (e.g., `feature/optimize-search-buffer` or `fix/editor-path-resolution`).

### 3. Coding Standards

* **Idiomatic Rust**: Follow `clippy` and `rustfmt` standards. Run `cargo clippy` before submitting any code.
* **Memory Efficiency**: Avoid external dependencies unless they are strictly necessary for core functionality.
* **Documentation**: All public functions must have doc-comments (`///`). Internal logic should be clarified with standard comments (`//`).

## Pull Request Process

1. **Fork the repository** and create your branch from `main`.
2. **Ensure the build passes**: Run `cargo build` and `cargo test` (if applicable).
3. **Update the Changelog**: If your change is notable, add a concise entry under the `[Unreleased]` section of `CHANGELOG.md`.
4. **Submit the PR**: Provide a clear description of the changes and the problem they solve.

## Bug Reports and Feature Requests

Please use the GitHub Issue tracker to report bugs or suggest enhancements. When reporting a bug, include:

* Your operating system and terminal environment.
* Steps to reproduce the issue.
* Expected vs. actual behavior.
