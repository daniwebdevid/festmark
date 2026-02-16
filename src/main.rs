// Copyright (c) 2024 Danydev
// Licensed under the MIT License.
//
// fsk (Fast Simple Knowledge): A minimalist CLI tool for managing markdown-based notes.
// This module handles CLI parsing and coordinates between the storage and editor layers.

mod cli;
mod storage;
mod editor;

use clap::Parser;
use cli::{Args, Commands};
use colored::*;

fn main() {
    // Initialize logger for debugging (controlled via RUST_LOG env var)
    env_logger::init();

    let args = Args::parse();

    // Linear command dispatching
    // Standard professional flow: Match -> Execute -> Handle Error
    match &args.cmd {
        // Creates or opens a note for writing using the system's default $EDITOR.
        Commands::Write { title } => {
            editor::editor(title);
        }

        // Retrieves and prints the raw content of a specific note.
        Commands::Get { title } => {
            match storage::read(title) {
                Ok(content) => {
                    if content.trim().is_empty() {
                        println!("{}", "Note is empty.".bright_black());
                    } else {
                        println!("{content}");
                    }
                }
                Err(e) => {
                    eprintln!(
                        "{} Failed to read '{}': {}", 
                        "✘".red(), 
                        title.yellow(), 
                        e.to_string().bright_black()
                    );
                }
            }
        }

        // Lists all available notes in the database recursively.
        Commands::List => {
            let files = storage::list();
            
            if files.is_empty() {
                println!("{} {}", "󰉖".red(), "No notes found in database.".bright_black());
                return;
            }

            println!("{} {}:", "󰠮".cyan(), "Your Knowledge Base".bold());
            println!("{}", "─".repeat(40).bright_black());
            
            for file in &files {
                println!("  {} {}", "•".blue(), file.bright_white());
            }
            
            println!("{}", "─".repeat(40).bright_black());
            println!("{} {} total notes", "󰇄".yellow(), files.len());
        }

        // Searches through titles and file contents for a specific keyword.
        Commands::Search { keyword } => {
            let results = storage::search(keyword);
    
            if results.is_empty() {
                println!("{} '{}'", "󰍉 No results found for".red(), keyword.yellow());
                return;
            }

            println!("{} '{}':", "󰥻 Found matches for".cyan(), keyword.yellow());
            println!("{}", "─".repeat(40).bright_black());

            for res in &results {
                if res.is_title_match {
                    println!("{} {}", "󰈚".blue(), res.title.bold().bright_white());
                } else {
                    println!("{} {}", "󰉈".green(), res.title.bright_white());
                    if let Some(text) = &res.preview {
                        println!("   {} {}", "↳".bright_black(), text.italic().bright_black());
                    }
                }
            }
            
            println!("{}", "─".repeat(40).bright_black());
            println!("{} {} result(s) found", "󰇄".yellow(), results.len());
        }
    }
}