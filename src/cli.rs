// Copyright (c) 2024 Danydev
// Licensed under the MIT License.
//
// fsk (Fast Simple Knowledge): Command-line argument definitions.
// Powered by Clap for efficient and type-safe parsing.

use clap::{Parser, Subcommand};

/// Fast Simple Knowledge (fsk) - A blazingly fast minimalist note-taking CLI.
/// 
/// fsk allows you to manage personal knowledge directly from the terminal
/// with support for nested directories and full-text search.
#[derive(Parser)]
#[command(name = "fsk", version = "0.1.0", author = "Danydev")]
#[command(about = "Manage your markdown notes with speed and simplicity", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,

    /// Enable verbose logging for debugging purposes.
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create or edit a note. Supports nested paths (e.g., 'linux/kernel').
    #[command(alias = "new", alias = "edit")]
    Write { 
        /// The title or path of the note (without .md extension).
        title: String 
    },

    /// Search for a keyword in titles and note contents.
    #[command(alias = "find")]
    Search { 
        /// The keyword or phrase to search for.
        keyword: String 
    },

    /// Display the content of a specific note to stdout.
    #[command(alias = "cat")]
    Get { 
        /// The title or path of the note to retrieve.
        title: String 
    },

    /// List all notes stored in the knowledge base.
    #[command(alias = "ls")]
    List,
}