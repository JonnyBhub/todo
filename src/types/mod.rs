// This module can be used for shared types and utilities in the future
// For now, our main types are in their respective modules:
// - Task: src/task.rs
// - CLI types: src/cli.rs
// - Storage: src/storage.rs
// - App: src/app.rs

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ValueEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
}