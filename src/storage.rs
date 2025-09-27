use crate::task::Task;
use std::fs;
use std::path::PathBuf;
use dirs;

pub struct Storage {
    file_path: String,
}

impl Storage {
    pub fn new() -> Self {
        let file_path = Self::get_data_file_path();
        let storage = Self { file_path };
        storage.ensure_data_directory();
        storage
    }

    fn get_data_file_path() -> String {
        let mut path = if let Some(data_dir) = dirs::data_dir() {
            // Use system data directory (e.g., ~/.local/share on Linux, ~/Library/Application Support on macOS)
            data_dir
        } else if let Some(home_dir) = dirs::home_dir() {
            // Fallback to home directory
            home_dir
        } else {
            // Last resort - current directory
            PathBuf::from(".")
        };
        
        path.push("todo-cli");
        path.push(".todo_data.json");
        path.to_string_lossy().to_string()
    }

    fn ensure_data_directory(&self) {
        let path = PathBuf::from(&self.file_path);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("Warning: Could not create data directory: {}", e);
                }
            }
        }
    }

    fn verify_file_integrity(&self) -> bool {
        // Basic integrity check - ensure file contains valid JSON and has expected structure
        if let Ok(contents) = fs::read_to_string(&self.file_path) {
            if let Ok(tasks) = serde_json::from_str::<Vec<Task>>(&contents) {
                // Additional checks could be added here (e.g., validate task IDs are sequential)
                return tasks.iter().all(|task| !task.description.is_empty());
            }
        }
        false
    }

    fn set_file_permissions(&self) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(file) = fs::File::open(&self.file_path) {
                if let Ok(metadata) = file.metadata() {
                    let mut perms = metadata.permissions();
                    perms.set_mode(0o600); // Read/write for owner only
                    let _ = fs::set_permissions(&self.file_path, perms);
                }
            }
        }
    }

    pub fn load_tasks(&self) -> Vec<Task> {
        if let Ok(contents) = fs::read_to_string(&self.file_path) {
            if !self.verify_file_integrity() {
                println!("Warning: Data file appears to be corrupted or tampered with");
                return Vec::new();
            }
            
            match serde_json::from_str::<Vec<Task>>(&contents) {
                Ok(tasks) => tasks,
                Err(_) => {
                    println!("Warning: could not parse tasks file, starting fresh");
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        }
    }

    pub fn save_tasks(&self, tasks: &[Task]) {
        match serde_json::to_string_pretty(tasks) {
            Ok(json) => {
                if let Err(e) = fs::write(&self.file_path, json) {
                    eprintln!("Warning: Could not save tasks: {}", e);
                } else {
                    // Set restrictive permissions after writing
                    self.set_file_permissions();
                }
            }
            Err(e) => eprintln!("Warning: Could not serialize tasks: {}", e),
        }
    }
}