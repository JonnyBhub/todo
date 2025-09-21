# Contributing to Todo CLI

Thank you for your interest in contributing to Todo CLI! This document provides guidelines and information for contributors.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or later
- Git
- A GitHub account (for submitting pull requests)

### Setting Up Your Development Environment

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/todo.git
   cd todo
   ```
3. Add the upstream repository as a remote:
   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/todo.git
   ```
4. Build the project to ensure everything works:
   ```bash
   cargo build
   ```
5. Run the tests:
   ```bash
   cargo test
   ```

## Development Workflow

### Before You Start

1. Check the [Issues](https://github.com/ORIGINAL_OWNER/todo/issues) page for existing bug reports or feature requests
2. If you're planning a significant change, open an issue first to discuss it
3. Make sure your fork is up to date with the upstream repository

### Making Changes

1. Create a new branch for your feature or bugfix:

   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bugfix-name
   ```

2. Make your changes, following the coding standards below

3. Test your changes thoroughly:

   ```bash
   cargo test
   cargo check
   cargo clippy -- -D warnings
   ```

4. Commit your changes with a clear, descriptive commit message:

   ```bash
   git commit -m "Add feature: task categories support"
   ```

5. Push your changes to your fork:

   ```bash
   git push origin feature/your-feature-name
   ```

6. Create a Pull Request on GitHub

## Coding Standards

### Rust Style Guidelines

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Use `cargo fmt` to format your code before committing
- Ensure `cargo clippy` passes without warnings
- Add documentation comments for public functions and modules
- Write unit tests for new functionality

### Code Structure

- Keep functions focused and small
- Use meaningful variable and function names
- Add error handling where appropriate
- Follow the existing patterns in the codebase

### Example Code Style

```rust
/// Adds a new task to the todo list
///
/// # Arguments
/// * `description` - The task description
/// * `due_date` - Optional due date in YYYY-MM-DD format
///
/// # Returns
/// * `Result<(), TodoError>` - Success or error details
pub fn add_task(&mut self, description: String, due_date: Option<String>) -> Result<(), TodoError> {
    // Implementation here
}
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Writing Tests

- Write unit tests for individual functions
- Write integration tests for command-line functionality
- Test edge cases and error conditions
- Use descriptive test names

Example test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task_with_valid_due_date() {
        let mut app = TodoApp::new();
        app.add_task("Test task".to_string(), Some("2025-12-31".to_string()));

        assert_eq!(app.tasks.len(), 1);
        assert_eq!(app.tasks[0].description, "Test task");
    }
}
```

## Documentation

- Update the README.md if you add new features
- Update the CHANGELOG.md following the established format
- Add inline documentation for complex functions
- Include examples in your documentation

## Pull Request Guidelines

### Before Submitting

- [ ] Code follows the project's style guidelines
- [ ] Tests pass locally
- [ ] New functionality is tested
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (if applicable)
- [ ] Commit messages are clear and descriptive

### Pull Request Description

Please include in your PR description:

1. **What**: Brief description of what you changed
2. **Why**: Explanation of why the change was needed
3. **How**: High-level description of how you implemented the change
4. **Testing**: How you tested the changes
5. **Screenshots**: If applicable, for UI changes

### Example PR Template

```markdown
## What

Added support for task categories to help organize tasks better.

## Why

Users requested the ability to categorize tasks (work, personal, etc.) for better organization.

## How

- Added `category` field to the Task struct
- Updated CLI to accept `--category` flag
- Added filtering by category in list command
- Updated JSON serialization to handle the new field

## Testing

- Added unit tests for category functionality
- Tested CLI commands manually
- Verified backward compatibility with existing task files

## Breaking Changes

None - new feature is optional and backward compatible.
```

## Issue Reporting

### Bug Reports

When reporting bugs, please include:

- **Description**: Clear description of the bug
- **Steps to Reproduce**: Detailed steps to reproduce the issue
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Environment**:
  - Operating System and version
  - Rust version (`rustc --version`)
  - Todo CLI version
- **Additional Context**: Any other relevant information

### Feature Requests

When requesting features, please include:

- **Description**: Clear description of the proposed feature
- **Use Case**: Why this feature would be useful
- **Examples**: How the feature would work (command examples, etc.)
- **Alternatives**: Any alternative solutions you've considered

## Code of Conduct

Please note that this project is released with a [Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.

## Questions?

If you have questions about contributing, feel free to:

- Open an issue with the `question` label
- Contact the maintainers directly
- Join our community discussions

Thank you for contributing to Todo CLI! 🚀
