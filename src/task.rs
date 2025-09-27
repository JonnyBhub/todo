use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub due_date: Option<NaiveDate>,
    pub completed_at: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(id: u32, description: String, due_date: Option<NaiveDate>) -> Self {
        Self {
            id,
            description,
            completed: false,
            due_date,
            completed_at: None,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
        self.completed_at = Some(Local::now());
    }

    pub fn is_urgent(&self, days_threshold: i64) -> bool {
        if self.completed {
            return false;
        }
        
        self.due_date.map_or(false, |due| {
            let today = Local::now().date_naive();
            let days_until_due = (due - today).num_days();
            days_until_due <= days_threshold
        })
    }

    pub fn is_overdue(&self) -> bool {
        if self.completed {
            return false;
        }
        
        self.due_date.map_or(false, |due| {
            let today = Local::now().date_naive();
            due < today
        })
    }

    pub fn matches_keyword(&self, keyword: &str) -> bool {
        self.description.to_lowercase().contains(&keyword.to_lowercase())
    }
}