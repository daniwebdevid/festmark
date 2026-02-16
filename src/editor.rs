// Copyright (c) 2024 Danydev
// Licensed under the MIT License.
//
// editor.rs: Interface for external text editors.
// Ensures directory existence and handles process lifecycle safely.

use std::process::Command;
use std::env;
use std::fs;
use log::{info, debug, error};
use crate::storage;

/// Launches the preferred editor to create or modify a note.
pub fn editor(title: &str) {
    // 1. Pelit RAM: Jangan alokasi String "nano" kalau env EDITOR udah ada.
    let editor_cmd = env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());

    let full_path = storage::resolve_path(title);

    // 2. Linear directory check
    if let Some(parent) = full_path.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                error!("Critical: Failed to create database directory: {e}");
                return;
            }
        }
    }

    let filepath_str = full_path.to_string_lossy();

    // 3. Execute and handle result
    if spawn_editor(&editor_cmd, &filepath_str) {
        info!("Note '{}' saved successfully.", title);
    } else {
        error!("Failed to launch editor '{}'. Check your $EDITOR environment variable.", editor_cmd);
    }
}

/// Spawns the editor process. Returns true if the process exited successfully.
fn spawn_editor(cmd: &str, filepath: &str) -> bool {
    debug!("Executing: {} {}", cmd, filepath);

    let status = Command::new(cmd)
        .arg(filepath)
        .status();

    match status {
        Ok(s) => s.success(),
        Err(e) => {
            error!("OS Error while launching editor: {e}");
            false
        }
    }
}