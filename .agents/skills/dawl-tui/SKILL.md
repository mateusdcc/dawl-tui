```markdown
# dawl-tui Development Patterns

> Auto-generated skill from repository analysis

## Overview
This skill teaches you the development patterns and conventions used in the `dawl-tui` Rust codebase. You'll learn about file naming, import/export styles, commit message patterns, and how to write and run tests. This guide is ideal for contributors aiming to maintain consistency and quality in this repository.

## Coding Conventions

### File Naming
- **Convention:** CamelCase is used for file names.
- **Example:**  
  ```
  myModule.rs
  anotherFile.rs
  ```

### Import Style
- **Convention:** Relative imports are preferred.
- **Example:**
  ```rust
  mod utils;
  use crate::utils::helper_function;
  ```

### Export Style
- **Convention:** Named exports are used.
- **Example:**
  ```rust
  pub fn my_function() { ... }
  pub struct MyStruct { ... }
  ```

### Commit Messages
- **Type:** Freeform (no strict structure)
- **Prefixes:** None required
- **Average Length:** ~39 characters
- **Example:**
  ```
  Fix bug in audio rendering pipeline
  Add support for new MIDI events
  ```

## Workflows

### Adding a New Module
**Trigger:** When you need to add a new feature or logical component.
**Command:** `/add-module`

1. Create a new file using CamelCase (e.g., `NewFeature.rs`).
2. Implement your module logic.
3. Use relative imports to include it in the main codebase.
4. Export public functions/structs with `pub`.
5. Write corresponding tests in a `.test.ts` file if applicable.
6. Commit your changes with a clear, concise message.

### Refactoring Existing Code
**Trigger:** When improving or restructuring code without changing its external behavior.
**Command:** `/refactor`

1. Identify the code to refactor.
2. Rename files using CamelCase if needed.
3. Update imports to use relative paths.
4. Ensure all exports remain named.
5. Run tests to verify no regressions.
6. Commit with a descriptive message.

### Writing and Running Tests
**Trigger:** When adding new features or fixing bugs.
**Command:** `/test`

1. Create or update a test file matching `*.test.ts`.
2. Write tests for your new or changed code.
3. Use the project's preferred test runner (framework unknown; check project docs or scripts).
4. Run the tests and ensure they pass.
5. Commit with a message indicating test changes.

## Testing Patterns

- **Framework:** Unknown (refer to project documentation or scripts).
- **File Pattern:** Test files are named with the `.test.ts` suffix.
- **Example:**
  ```
  myModule.test.ts
  ```
- **Best Practice:** Place tests alongside the code they cover and ensure comprehensive coverage for new features and bug fixes.

## Commands
| Command        | Purpose                                         |
|----------------|-------------------------------------------------|
| /add-module    | Scaffold and add a new module                   |
| /refactor      | Refactor existing code following conventions    |
| /test          | Write and run tests for your code               |
```