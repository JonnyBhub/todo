use chrono::{DateTime, Local, NaiveDate};
use clap::{Parser, Subcommand};
use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Effects};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().bold())
    .error(AnsiColor::Red.on_default().bold())
    .usage(AnsiColor::Green.on_default().bold())
    .literal(AnsiColor::Blue.on_default().bold())
    .placeholder(AnsiColor::Cyan.on_default());


#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"), author = env!("CARGO_PKG_AUTHORS"))]
#[command(about = "A simple CLI todo manager - add, edit, list, search, complete, and remove tasks", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(propagate_version = true)]
#[command(styles = STYLES)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    #[command(visible_aliases = ["+", "a"])]
    Add {
        /// Task description
        #[arg(required = true)]
        description: String,
        /// Optional due date in YYYY-MM-DD format
        #[arg(short, long)]
        due: Option<String>,
    },
    /// Edit an existing task by ID, you can change the description and/or due date
    Edit {
        /// Task ID
        #[arg(required = true)]
        id: u32,
        /// New task description
        description: Option<String>,
        /// Optional new due date in YYYY-MM-DD format
        #[arg(short, long)]
        due: Option<String>,
    },
    /// List all tasks use --urgent, -u to filter for tasks due within 3 days
    List {
        /// Show only tasks due soon ( within 3 days)
        #[arg(short, long)]
        urgent: bool,
    },
    /// Search tasks by keyword
    Search {
        /// Keyword to search for in task descriptions
        keyword: String,
    },
    /// Mark a task as complete
    Complete {
        /// Task ID
        id: u32,
    },
    /// Mark multiple tasks as complete, provide a list of IDs
    CompleteTasks {
        /// List of Task IDs to complete
        ids: Vec<u32>,
    },
    /// Remove a task
    #[command(visible_aliases = ["-","rm","del"])]
    Remove { id: u32 },
    /// Remove all tasks
    /// Use with caution!
    /// This will delete all tasks permanently.
    RemoveAll,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
    due_date: Option<NaiveDate>,
    completed_at: Option<DateTime<Local>>,
}

struct TodoApp {
    tasks: Vec<Task>,
    next_id: u32,
    file_path: String,
}

impl TodoApp {
    fn new() -> Self {
        let mut app = TodoApp {
            tasks: Vec::new(),
            next_id: 1,
            file_path: Self::get_data_file_path(),
        };
        app.ensure_data_directory();
        app.load_tasks();
        app
    }

    fn get_data_file_path() -> String {
        let mut path = if let Some(data_dir) = dirs::data_dir() {
            data_dir
        }
        else if let Some(home_dir) = dirs::home_dir() {
            home_dir
        }
        else {
            PathBuf::from(".")
        };

        path.push("todo-cli");
        path.push(".todo_data.json");
        return path.to_string_lossy().to_string();
    }


    fn ensure_data_directory(&self) {
        if let Some(parent) = PathBuf::from(&self.file_path).parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("Warning: Could not create data directory: {}", e);
                }
            }
        }
    }

    fn verify_file_integrity(&self) -> bool {
        if let Ok(contents) = fs::read_to_string(&self.file_path) {
            if let Ok(tasks) = serde_json::from_str::<Vec<Task>>(&contents){
                return tasks.iter().all(|task|!task.description.is_empty());
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
                    perms.set_mode(0o600); // Read/write for owner
                    let _ = fs::set_permissions(&self.file_path, perms);
                }
            }

        }
    }

    fn add_task(&mut self, description: String, due_date_str: Option<String>) {
        let due_date =
            due_date_str.and_then(|date_str| NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok());

        if due_date.is_some() && due_date.is_none() {
            println!("Warning: Invalid due date format. Use YYYY-MM-DD.");
            return;
        }

        let task = Task {
            id: self.next_id,
            description,
            completed: false,
            due_date,
            completed_at: None,
        };

        self.tasks.push(task);
        self.next_id += 1;
        self.save_tasks();

        println!(
            "Added task #{}: {}",
            self.next_id - 1,
            self.tasks.last().unwrap().description
        );
    }

    fn edit_task(&mut self, id: u32, new_desc: Option<String>, due_date: Option<String>) {
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                if let Some(description) = new_desc {
                    task.description = description;
                }
                if let Some(due) = due_date
                    .and_then(|date_str| NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok())
                {
                    task.due_date = Some(due);
                }
                let edited_description = task.description.clone();
                let edited_due_date = task.due_date;
                self.save_tasks();
                println!(
                    "Edited task #{}: {}. Due - {}",
                    id,
                    edited_description,
                    edited_due_date.map_or("No due date".to_string(), |d| d.to_string())
                );
            }
            None => println!("Task #{} not found", id),
        }
    }

    fn list_tasks(&self, urgent_only: bool) {
        let mut tasks_to_show: Vec<&Task> = if urgent_only {
            let today = Local::now().date_naive();

            self.tasks
                .iter()
                .filter(|task| {
                    !task.completed
                        && task.due_date.is_some_and(|due| {
                            let days_until_due = (due - today).num_days();
                            days_until_due <= 3
                        })
                })
                .collect()
        } else {
            self.tasks.iter().collect()
        };

        if tasks_to_show.is_empty() {
            if urgent_only {
                println!("No urgent tasks due within the next 3 days!");
            } else {
                println!("No tasks found!");
            }
            return;
        }

        tasks_to_show.sort_by(|a, b| {
            let today = Local::now().date_naive();

            match (a.due_date, b.due_date) {
                (Some(ad), Some(bd)) => {
                    let a_days = (ad - today).num_days();
                    let b_days = (bd - today).num_days();
                    a_days.cmp(&b_days)
                }
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });

        let title = if urgent_only {
            "Urgent tasks:"
        } else {
            "Your tasks:"
        };
        println!("{}", title);

        let today = Local::now().date_naive();

        for task in tasks_to_show {
            let status = if task.completed { "✓" } else { " " };
            let urgency_indicator = match task.due_date {
                Some(due) => {
                    let days_until = (due - today).num_days();
                    if days_until < 0 {
                        format!(" 🔴 OVERDUE by {} days", -days_until)
                    } else {
                        match days_until {
                            0 => " 🟡 DUE TODAY".to_string(),
                            1 => " 🟠 Due tomorrow".to_string(),
                            2..=3 => format!(" 🟡 Due in {} days", days_until),
                            4..=7 => format!(" (due {})", due.format("%m-%d")),
                            _ => format!(" (due {})", due.format("%Y-%m-%d")),
                        }
                    }
                }
                None => String::new(),
            };

            println!(
                "[{}] {}: {}{}",
                status, task.id, task.description, urgency_indicator
            );
        }
    }

    fn search_tasks(&self, keyword: &str) {
        let keyword_lower = keyword.to_lowercase();
        let matching_tasks: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|task| task.description.to_lowercase().contains(&keyword_lower))
            .collect();

        if matching_tasks.is_empty() {
            println!("No tasks found matching '{}'", keyword);
            return;
        }

        println!("Tasks matching '{}':", keyword);
        for task in matching_tasks {
            let status = if task.completed { "✓" } else { " " };
            println!(
                "[{}] {}: {}. Due - {}",
                status,
                task.id,
                task.description,
                task.due_date
                    .map_or("No due date".to_string(), |d| d.to_string())
            );
        }
    }
    fn complete_task(&mut self, id: u32) {
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                task.completed = true;
                task.completed_at = Some(Local::now());
                self.save_tasks();
                println!("Completed task #{}", id);
            }
            None => println!("Task #{} not found", id),
        }
    }

    fn remove_task(&mut self, id: u32) {
        let initial_len = self.tasks.len();
        self.tasks.retain(|task| task.id != id);

        if self.tasks.len() < initial_len {
            self.save_tasks();
            println!("Removed task #{}", id);
        } else {
            println!("Task #{} not found", id);
        }
    }

    fn load_tasks(&mut self) {
        if let Ok(contents) = fs::read_to_string(&self.file_path) { 
            if !self.verify_file_integrity() {
                println!("Warning: Data file appears to be corrupted or tampered with");
                return;
            }
            match serde_json::from_str::<Vec<Task>>(&contents) {
            Ok(tasks) => {
                self.tasks = tasks;
                self.next_id = self.tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
            }
            Err(_) => println!("Warning: could not parse tasks file, starting fresh"),
        } }
    }

    fn save_tasks(&self) {
        match serde_json::to_string_pretty(&self.tasks) {
            Ok(json) => {
                if let Err(e) = fs::write(&self.file_path, json) {
                    eprintln!("Warning: Could not save tasks: {}", e);
                } else {
                    // Sets the permissions after writing
                    self.set_file_permissions();
                }
            }
            Err(e) => eprintln!("Warning: Could not serialize tasks: {}", e),
        }
    }

    fn remove_all_tasks(&mut self) {
        self.tasks.clear();
        self.next_id = 1;
        self.save_tasks();
        println!("All tasks have been removed.");
    }
}

fn main() {
    let cli = Cli::parse();
    let mut app = TodoApp::new();

    match cli.command {
        Commands::Add { description, due } => {
            app.add_task(description, due);
        }
        Commands::Edit {
            id,
            description,
            due,
        } => {
            app.edit_task(id, description, due);
        }
        Commands::List { urgent } => {
            app.list_tasks(urgent);
        }
        Commands::Search { keyword } => {
            app.search_tasks(&keyword);
        }
        Commands::Complete { id } => {
            app.complete_task(id);
        }
        Commands::CompleteTasks { ids } => {
            for id in ids {
                app.complete_task(id);
            }
        }
        Commands::Remove { id } => {
            app.remove_task(id);
        }
        Commands::RemoveAll => {
            app.remove_all_tasks();
        }
    }
}
