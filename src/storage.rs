// Copyright (c) 2024 Danydev
// Licensed under the MIT License.
//
// storage.rs: Data access layer. Handles file I/O and recursive directory traversal
// with a focus on memory efficiency and linear execution.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Resolves the absolute path to a note. 
/// If title is empty, returns the base database directory.
pub fn resolve_path(title: &str) -> PathBuf {
    // Priority: $HOME/.fsk/db or fallback to current directory
    let base = env::var("HOME")
        .map(|h| PathBuf::from(h).join(".fsk").join("db"))
        .unwrap_or_else(|_| PathBuf::from("./db"));

    if title.is_empty() {
        base
    } else {
        base.join(format!("{}.md", title))
    }
}

/// Reads note content. Uses standard Result for professional error propagation.
pub fn read(title: &str) -> Result<String, std::io::Error> {
    let path = resolve_path(title);
    fs::read_to_string(path)
}

/// Recursively lists all .md files. 
/// Uses a simple internal recursive function to avoid heavy external dependencies.
pub fn list() -> Vec<String> {
    let db_path = resolve_path("");
    let mut files = Vec::new();
    
    visit_dirs(&db_path, &db_path, &mut files);
    files.sort();
    files
}

/// Full-text search optimized for memory.
/// Does not load file content if the title already matches the keyword.
pub struct SearchResult {
    pub title: String,
    pub is_title_match: bool,
    pub preview: Option<String>,
}

pub fn search(keyword: &str) -> Vec<SearchResult> {
    let db_path = resolve_path("");
    let kw_lower = keyword.to_lowercase(); // Allocated once
    let mut results = Vec::new();
    
    // Standard recursive walker logic but simplified
    walk_and_search(&db_path, &db_path, &kw_lower, &mut results);
    results
}

// --- Private Helpers (The "Linear & Clean" way) ---

fn visit_dirs(dir: &Path, base: &Path, acc: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            visit_dirs(&path, base, acc);
        } else if path.extension().map_or(false, |e| e == "md") {
            if let Ok(rel) = path.strip_prefix(base) {
                acc.push(rel.with_extension("").to_string_lossy().to_string());
            }
        }
    }
}

fn walk_and_search(dir: &Path, base: &Path, kw: &str, results: &mut Vec<SearchResult>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    
    for entry in entries.flatten() {
        let path = entry.path();
        
        if path.is_dir() {
            walk_and_search(&path, base, kw, results);
            continue;
        }

        if path.extension().map_or(false, |e| e == "md") {
            let rel_path = path.strip_prefix(base).unwrap_or(&path);
            let title = rel_path.with_extension("").to_string_lossy().to_string();
            
            // 1. Title Match (Fast Path)
            if title.to_lowercase().contains(kw) {
                results.push(SearchResult {
                    title,
                    is_title_match: true,
                    preview: None,
                });
                continue; // Skip reading file content to save RAM/IO
            }

            // 2. Content Match (Lazy Loading)
            if let Ok(content) = fs::read_to_string(&path) {
                if let Some(line) = content.lines().find(|l| l.to_lowercase().contains(kw)) {
                    results.push(SearchResult {
                        title,
                        is_title_match: false,
                        preview: Some(line.trim().to_string()),
                    });
                }
            }
        }
    }
}