# todo-cli

A simple command-line todo list manager written in Rust. Tasks are stored in a local `todo.json` file.

## Features

- Add tasks
- List all tasks
- Mark tasks as finished (removes them)
- Data is persisted in `todo.json`
- Interactive REPL-style prompt

## Usage

1. **Build and run:**
   ```sh
   cargo run
   ```

2. **Commands:**
   - `add <task description>`  
     Adds a new task.  
     Example:  
     ```
     todo> add Buy groceries
     ```
   - `print`  
     Lists all tasks.
   - `finish <task number>`  
     Marks a task as finished (removes it by its number as shown in `print`).  
     Example:  
     ```
     todo> finish 2
     ```
   - `stop` or `exit`  
     Exits the program.

## Example Session

```
todo> add Learn Rust
Task added.
todo> add Write a todo app
Task added.
todo> print
Tasks:
1: Learn Rust
2: Write a todo app
todo> finish 1
Finished task: Learn Rust
todo> print
Tasks:
1: Write a todo app
todo> stop
Exiting...
```

## Notes

- Tasks are saved in `todo.json` in the project directory.
- `todo.json` is ignored by git (see `.gitignore`).

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json) crates (already in