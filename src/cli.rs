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
#[command(name = "fsk", version = env!("CARGO_PKG_VERSION"), author = "Danydev")]
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

    /// Remove a note from the database.
    #[command(alias = "rm", alias = "del", alias = "delete")]
    Remove { title: String },

    /// Move or rename a note (supports subfolders).
    #[command(alias = "rename", alias = "mv")]
    Move { from: String, to: String },

    /// Export a folder or all notes to a specific destination.
    #[command(alias = "bundle")]
    Export { 
        /// The folder to export (use "." or "all" for everything).
        folder: String,
        /// The destination path (e.g., "~/Documents/my_archive").
        destination: String 
    },

    /// Import notes from an external folder into the database.
    #[command(alias = "add-archive")]
    Import { 
        /// The path to the folder you want to import.
        source: String 
    },

    /// List all notes or notes within a specific folder.
    #[command(alias = "ls")]
    List {
        /// Optional: The folder/path to list (e.g., 'a7x')
        path: Option<String>,
    },
}
