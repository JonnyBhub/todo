# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned Features

- Interactive mode for better user experience
  - https://github.com/ratatui/ratatui/blob/main/examples/apps/todo-list/src/main.rs
- Task categories and tags
- Priority levels for tasks
- Task dependencies
- Export tasks to different formats (CSV, Markdown)
- Recurring tasks support

## [0.1.0] - 2025-09-21

### Added

- Initial release of Todo CLI
- Basic task management (add, edit, remove, complete)
- Due date support with YYYY-MM-DD format
- Visual indicators for task urgency:
  - 🔴 Overdue tasks
  - 🟡 Tasks due today
  - 🟠 Tasks due tomorrow
  - 🟡 Tasks due within 3 days
- Urgent task filtering (tasks due within 3 days)
- Task search functionality by keyword
- Bulk task completion with `complete-tasks` command
- Persistent storage using JSON format
- Automatic task ID management
- Command-line interface using `clap` crate
- Task sorting by due date priority

### Technical Details

- Built with Rust 2024 edition
- Uses `serde` for JSON serialization
- Uses `chrono` for date handling
- Uses `clap` for CLI argument parsing
- Tasks stored in `tasks.json` in working directory
