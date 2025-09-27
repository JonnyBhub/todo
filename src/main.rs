mod app;
mod cli;
mod storage;
mod task;
mod types;

use clap::Parser;
use cli::{Cli, Commands};
use app::TodoApp;

fn main() {
    let cli = Cli::parse();
    let mut app = TodoApp::new();

    match cli.command {
        Commands::Add { description, priority, tags, due } => {
            app.add_task(description, priority, tags, due);
        }
        Commands::Edit { id, description, due } => {
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
        Commands::Completions { shell } => {
            Cli::generate_completions(shell);
        }
    }
}
