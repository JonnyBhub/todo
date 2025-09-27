use clap::{CommandFactory, Parser, Subcommand};
use clap::builder::Styles;
use clap::builder::styling::AnsiColor;
use clap_complete::{generate, Shell};
use std::io;
use crate::types::Priority;


const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().bold().underline())
    .error(AnsiColor::Red.on_default().bold())
    .usage(AnsiColor::Green.on_default().bold())
    .literal(AnsiColor::Blue.on_default().bold())
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"), author = "Jonathan Barrett <psveagle@pm.me>")]
#[command(about = "A simple CLI todo manager")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(styles = STYLES)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    #[command(visible_aliases = ["+", "a"])]
    Add {
        /// Task description
        description: String,
        /// Optional due date in YYYY-MM-DD format
        
        #[arg(short,long, value_enum)]
        priority: Option<Priority>,

        /// Optional tags, comma-separated
        #[arg(short, long)]
        tags: Option<String>,

        #[arg(short, long)]
        due: Option<String>,


    },
    /// Edit an existing task by ID, you can change the description and/or due date
    Edit {
        /// Task ID
        id: u32,
        /// New task description
        description: Option<String>,
        /// Optional new due date in YYYY-MM-DD format
        #[arg(short, long)]
        due: Option<String>,
    },
    /// List all tasks
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
    /// Remove a task by ID
    #[command(visible_aliases = ["-", "rm", "del"])]
    Remove { id: u32 },
    /// Remove all tasks
    /// Use with caution!
    /// This will delete all tasks permanently.
    RemoveAll,
    /// Generate shell completions
    #[command(visible_aliases = ["comp"])]
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

impl Cli {
    pub fn generate_completions(shell: Shell) {
        let mut cmd = Self::command();
        let bin_name = cmd.get_name().to_string();
        generate(shell, &mut cmd, bin_name, &mut io::stdout());
    }
}