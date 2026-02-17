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

/// Recursively lists notes. Supports filtering by a sub-path.
pub fn list(sub_path: Option<&String>) -> Vec<String> {
    let db_path = resolve_path("");
    
    let start_path = match sub_path {
        Some(p) => db_path.join(p),
        None => db_path.clone(),
    };

    let mut files = Vec::new();
    
    if start_path.exists() && start_path.is_dir() {
        visit_dirs(&start_path, &db_path, &mut files);
    }
    
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

/// Deletes a note or an entire directory from the database.
/// If a note is deleted, it automatically performs a recursive cleanup of empty parent directories.
pub fn remove(title: &str) -> std::io::Result<()> {
    let db_path = resolve_path(""); 
    let path_with_ext = db_path.join(format!("{}.md", title));
    let path_as_dir = db_path.join(title);

    if path_with_ext.is_file() {
        fs::remove_file(&path_with_ext)?;
        
        // --- Recursive Auto-Cleanup ---
        // Ensures the database stays lean by pruning empty parent folders.
        let mut current = path_with_ext.parent();
        while let Some(dir) = current {
            if dir == db_path { break; } // Safety: Never delete the root database
            
            // If the directory is empty, remove it and move to the parent
            if fs::read_dir(dir)?.next().is_none() {
                fs::remove_dir(dir)?;
                current = dir.parent();
            } else {
                break; 
            }
        }
        Ok(())
    } else if path_as_dir.is_dir() {
        // Targeted removal of a folder and all its content (The "Nuclear" option)
        fs::remove_dir_all(path_as_dir)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound, 
            format!("Note or folder '{}' not found", title)
        ))
    }
}

/// Renames or moves a note. 
/// Automatically creates parent directories if the destination is a new subfolder.
pub fn rename(from: &str, to: &str) -> std::io::Result<()> {
    let path_from = resolve_path(from);
    let path_to = resolve_path(to);

    // Ensure the destination subdirectory exists (supports fsk mv note folder/note)
    if let Some(parent) = path_to.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    fs::rename(path_from, path_to)
}

/// Exports a folder or the entire database to a destination path.
/// If source_title is empty, it exports the entire knowledge base.
pub fn export_folder(source_title: &str, destination_path: &str) -> std::io::Result<()> {
    let db_path = resolve_path("");
    let src = if source_title.is_empty() {
        db_path.clone()
    } else {
        db_path.join(source_title)
    };
    
    let dest = PathBuf::from(destination_path);

    if !src.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound, 
            format!("Source '{}' not found in database", source_title)
        ));
    }

    // Standard recursive copy implementation
    copy_dir_recursive(&src, &dest)
}

/// Internal helper for recursive directory copying.
/// Ensures nested structures are preserved during export/import.
fn copy_dir_recursive(src: &Path, dest: &Path) -> std::io::Result<()> {
    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}
