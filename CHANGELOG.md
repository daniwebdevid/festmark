# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-02-16

### Added

* **Minimalist Storage Engine**: Implementation of a file-based database located at `~/.fsk/db`.
* **Recursive Directory Support**: Built-in support for nested markdown files (e.g., `linux/kernel`).
* **Lazy Search Algorithm**: Optimized search flow that prioritizes file titles before content scanning to reduce I/O overhead.
* **System Editor Integration**: Seamless interfacing with `$EDITOR` environment variable with built-in fallbacks.
* **Semantic CLI Interface**: Robust command-line parsing using `clap` with support for aliases (`ls`, `cat`, `find`).
* **Memory Efficient Architecture**: Designed with a zero-dependency mindset for core directory traversal to ensure a small binary footprint.
* **Automated Workspace Setup**: Self-provisioning of the necessary directory structures on the first write operation.
* **Professional Error Reporting**: Standardized error output for missing files, permission issues, and process failures.
