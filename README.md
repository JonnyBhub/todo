# Todo CLI

A simple and efficient command-line todo list manager written in Rust. Keep track of your tasks with due dates, priorities, and completion status.

## Features

- ✅ Add, edit, and remove tasks
- 📅 Set due dates for tasks
- 🔍 Search tasks by keyword
- ⚡ View urgent tasks (due within 3 days)
- 📊 Visual indicators for overdue and upcoming tasks
- 💾 Persistent storage in JSON format
- 🚀 Fast and lightweight CLI interface

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (1.70 or later)

### Building from Source

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd todo
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. The executable will be available at `target/release/todo`

### Installing Globally

To install the todo CLI globally on your system:

```bash
cargo install --path .
```

This will install the `todo` command to your Cargo bin directory (usually `~/.cargo/bin/`).

## Usage

### Adding Tasks

Add a simple task:

```bash
todo add "Buy groceries"
```

Add a task with a due date:

```bash
todo add "Submit report" --due 2025-09-25
```

### Listing Tasks

List all tasks:

```bash
todo list
```

List only urgent tasks (due within 3 days):

```bash
todo list --urgent
```

### Editing Tasks

Edit a task's description:

```bash
todo edit 1 "Buy groceries and cook dinner"
```

Edit a task's due date:

```bash
todo edit 1 --due 2025-09-22
```

Edit both description and due date:

```bash
todo edit 1 "Complete project proposal" --due 2025-09-30
```

### Searching Tasks

Search for tasks containing a keyword:

```bash
todo search "groceries"
```

### Completing Tasks

Mark a single task as complete:

```bash
todo complete 1
```

Mark multiple tasks as complete:

```bash
todo complete-tasks 1 2 3
```

### Removing Tasks

Remove a specific task:

```bash
todo remove 1
```

Remove all tasks (use with caution):

```bash
todo remove-all
```

## Visual Indicators

The todo CLI uses visual indicators to help you prioritize your tasks:

- 🔴 **OVERDUE** - Tasks that are past their due date
- 🟡 **DUE TODAY** - Tasks due today
- 🟠 **Due tomorrow** - Tasks due tomorrow
- 🟡 **Due in X days** - Tasks due within 3 days
- ✓ **Completed** - Completed tasks

## Data Storage

Tasks are stored in a `tasks.json` file in the current working directory. The data includes:

- Task ID
- Description
- Completion status
- Due date (optional)
- Completion timestamp (when marked as complete)

## Examples

Here's a typical workflow:

```bash
# Add some tasks
todo add "Review pull requests" --due 2025-09-22
todo add "Update documentation"
todo add "Plan team meeting" --due 2025-09-23

# List all tasks
todo list

# Mark urgent tasks
todo list --urgent

# Complete a task
todo complete 1

# Search for specific tasks
todo search "meeting"

# Edit a task
todo edit 2 "Update API documentation" --due 2025-09-25
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature/new-feature`)
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and updates.
