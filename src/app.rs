use crate::{task::Task, types::Priority};
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

    pub fn add_task(&mut self, description: String, priority_input:Option<Priority>, tag_list:Option<String>, due_date_str: Option<String>) {        
        let due_date = due_date_str.clone().and_then(|date_str| 
            NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok()
        );

        if due_date_str.is_some() && due_date.is_none() {
            println!("Warning: Invalid due date format. Use YYYY-MM-DD.");
            return;
        }

        let mut priority = None;

        if let Some(priority_value) = priority_input {
            priority = Some(priority_value);
        }

        let mut tags = None;
        if let Some(tag_string) = tag_list {
            tags = Some(tag_string
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>());
        }

        let task = Task::new(self.next_id, description, priority, tags, due_date);
        self.tasks.push(task);
        self.next_id += 1;
        self.storage.save_tasks(&self.tasks);

        println!("Added task #{}: {}", self.next_id - 1, self.tasks.last().unwrap().description);
    }

    pub fn edit_task(&mut self, id: u32, new_desc: Option<String>, priority_input: Option<Priority>, tag_list_replace: Option<String>, tag_list_add: Option<String>, due_date: Option<String>) {
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                // perform updates while we have the mutable borrow and collect owned display values,
                // then the borrow will end at the end of this inner block so we can call save_tasks().
                let (desc_owned, due_owned, priority_owned, tags_owned) = {
                    if let Some(description) = new_desc {
                        task.description = description;
                    }

                    if let Some(priority_value) = priority_input {
                        task.priority = Some(priority_value);
                    }

                    // Replace tags if provided
                    if let Some(tag_string) = tag_list_replace {
                        let parsed: Vec<String> = tag_string
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        task.tags = if parsed.is_empty() { None } else { Some(parsed) };
                    }

                    // Append tags if provided
                    if let Some(add_string) = tag_list_add {
                        let parsed_to_add: Vec<String> = add_string
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        if !parsed_to_add.is_empty() {
                            match &mut task.tags {
                                Some(existing) => {
                                    existing.extend(parsed_to_add.into_iter());
                                    // optional: dedupe
                                    existing.sort();
                                    existing.dedup();
                                }
                                None => {
                                    task.tags = Some(parsed_to_add);
                                }
                            }
                        }
                    }

                    if let Some(due) = due_date.and_then(|date_str| NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").ok()) {
                        task.due_date = Some(due);
                    }

                    // take owned copies for printing after we drop the mutable borrow
                    let desc_owned = task.description.clone();
                    let due_owned = task.due_date.map(|d| d.to_string());
                    let priority_owned = match &task.priority {
                        Some(p) => match p {
                            Priority::High => "🔴 HIGH".to_string(),
                            Priority::Medium => "🟡 MED".to_string(),
                            Priority::Low => "🟢 LOW".to_string(),
                        },
                        None => "None".to_string(),
                    };
                    let tags_owned = task.tags.as_ref().map(|v| v.join(", ")).unwrap_or_else(|| "No tags".to_string());

                    (desc_owned, due_owned, priority_owned, tags_owned)
                }; // mutable borrow of `task` ends here

                self.storage.save_tasks(&self.tasks);

                println!("Edited task #{}: {}\n  Due: {}\n  Priority: {}\n  Tags: {}", 
                    id, 
                    desc_owned, 
                    due_owned.unwrap_or_else(|| "No due date".to_string()),
                    priority_owned,
                    tags_owned
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
        println!("{}\n", title);
        
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


            // format priority
            let priority_display = match &task.priority {
                Some(p) => match p {
                    Priority::High => "🔴 HIGH".to_string(),
                    Priority::Medium => "🟡 MED".to_string(),
                    Priority::Low => "🟢 LOW".to_string(),
                },
                None => "None".to_string(),
            };

            // format tags (unwrap Option<Vec<String>> into a human string)
            let tags_display = match &task.tags {
                Some(tags) if !tags.is_empty() => tags.join(", "),
                _ => "No tags".to_string(),
            };

            println!("[{}] {}: {}{}.\n  Priority: {}\n  Tags: {}\n", status, task.id, task.description, urgency_indicator, priority_display, tags_display);
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