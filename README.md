# fsk (festmark)

A minimalist, high-performance command-line interface (CLI) for managing markdown-based knowledge bases. Built with Rust for memory safety and zero-cost abstractions.

## Overview

**fsk** is designed for users who require a frictionless, terminal-centric workflow for note-taking and documentation. It adheres to the Unix philosophy: doing one thing and doing it well. The system operates on a flat or nested directory structure of standard Markdown files, ensuring portability and long-term data sovereignty.

## Core Features

* **Recursive Navigation**: Full support for nested directory structures (e.g., `linux/kernel`).
* **Lazy Search Engine**: Optimized full-text search that prioritizes filename matching before initiating file I/O to minimize memory overhead.
* **Editor Agnostic**: Interfaces seamlessly with any system-defined text editor via the `$EDITOR` environment variable.
* **Zero Dependencies (Core Logic)**: Utilizes the Rust standard library for directory traversal to maintain a small binary footprint.

## Installation

### Prerequisites

* Rust Toolchain (Cargo 1.80+)
* A Unix-like environment (Linux, macOS, or BSD)

### Build from Source

```bash
git clone https://github.com/yourusername/fsk.git
cd fsk
cargo build --release

```

The compiled binary will be located at `target/release/fsk`. For system-wide access, move the binary to `/usr/local/bin/`.

## Configuration

**fsk** stores data in the following directory:
`$HOME/.fsk/db/`

The application will attempt to create this directory automatically upon the first execution of the `write` command. Ensure the environment has appropriate write permissions for this path.

## Usage

### Managing Notes

Create or edit a note by providing a title. Nested paths are automatically handled.

```bash
fsk write linux/networking

```

### Retrieving Content

Output the raw content of a note directly to stdout.

```bash
fsk get linux/networking

```

### Searching the Knowledge Base

Search for keywords across all file titles and content.

```bash
fsk search "iptables"

```

### Listing All Notes

Display a sorted tree structure of the current database.

```bash
fsk list

```

## Technical Architecture

The project is modularized into four distinct layers:

1. **CLI Layer (`cli.rs`)**: Handles argument parsing and command-line interface definitions using the Clap crate.
2. **Storage Layer (`storage.rs`)**: Manages the file system abstraction, including recursive traversal and memory-efficient search algorithms.
3. **Editor Layer (`editor.rs`)**: Manages the lifecycle of external editor processes and directory integrity.
4. **Main Entry (`main.rs`)**: Orchestrates the execution flow and handles professional error reporting.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.