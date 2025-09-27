use crate::task::Task;
use crate::storage::Storage;
use chrono::{Local, NaiveDate};

pub struct TodoApp {
    tasks: Vec<Task>,
    next_id: u32,
    storage: Storage,
}

impl TodoApp {
    pub fn new() -> Self {
        let storage = Storage::new();
        let tasks = storage.load_tasks();
        let next_id = tasks.iter()
            .map(|task| task.id)
            .max()
            .unwrap_or(0) + 1;

        Self {
            tasks,
            next_id,
            storage,
        }
    }

    pub fn add_task(&mut self, description: String, due_date_str: Option<String>) {
        let due_date = due_date_str.clone().and_then(|date_str| 
            NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok()
        );

        if due_date_str.is_some() && due_date.is_none() {
            println!("Warning: Invalid due date format. Use YYYY-MM-DD.");
            return;
        }

        let task = Task::new(self.next_id, description, due_date);
        self.tasks.push(task);
        self.next_id += 1;
        self.storage.save_tasks(&self.tasks);

        println!("Added task #{}: {}", self.next_id - 1, self.tasks.last().unwrap().description);
    }

    pub fn edit_task(&mut self, id: u32, new_desc: Option<String>, due_date: Option<String>) {
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                if let Some(description) = new_desc {
                    task.description = description;
                }
                if let Some(due) = due_date.and_then(|date_str| 
                    NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok()
                ) {
                    task.due_date = Some(due);
                }
                let edited_description = task.description.clone();
                let edited_due_date = task.due_date;
                self.storage.save_tasks(&self.tasks);
                println!("Edited task #{}: {}. Due - {}", 
                    id, 
                    edited_description, 
                    edited_due_date.map_or("No due date".to_string(), |d| d.to_string())
                );
            }
            None => println!("Task #{} not found", id),
        }
    }

    pub fn list_tasks(&self, urgent_only: bool) {
        let mut tasks_to_show: Vec<&Task> = if urgent_only {
            self.tasks.iter().filter(|task| task.is_urgent(3)).collect()
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

        let title = if urgent_only { "Urgent tasks:" } else { "Your tasks:" };
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

            println!("[{}] {}: {}{}", status, task.id, task.description, urgency_indicator);
        }
    }

    pub fn search_tasks(&self, keyword: &str) {
        let matching_tasks: Vec<&Task> = self.tasks.iter()
            .filter(|task| task.matches_keyword(keyword))
            .collect();

        if matching_tasks.is_empty() {
            println!("No tasks found matching '{}'", keyword);
            return;
        }

        println!("Tasks matching '{}':", keyword);
        for task in matching_tasks {
            let status = if task.completed { "✓" } else { " " };
            println!("[{}] {}: {}. Due - {}", 
                status, 
                task.id, 
                task.description, 
                task.due_date.map_or("No due date".to_string(), |d| d.to_string())
            );
        }
    }

    pub fn complete_task(&mut self, id: u32) {
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                task.complete();
                self.storage.save_tasks(&self.tasks);
                println!("Completed task #{}", id);
            }
            None => println!("Task #{} not found", id),
        }
    }

    pub fn remove_task(&mut self, id: u32) {
        let initial_len = self.tasks.len();
        self.tasks.retain(|task| task.id != id);
        
        if self.tasks.len() < initial_len {
            self.storage.save_tasks(&self.tasks);
            println!("Removed task #{}", id);
        } else {
            println!("Task #{} not found", id);
        }
    }

    pub fn remove_all_tasks(&mut self) {
        self.tasks.clear();
        self.next_id = 1;
        self.storage.save_tasks(&self.tasks);
        println!("All tasks have been removed.");
    }
}