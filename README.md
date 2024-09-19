# Terminal TODO App

A simple, interactive terminal TODO app written in Rust. You are able to manage your tasks with separate TODO and DONE lists, all within your terminal.

## Features

- Add, edit, and delete tasks
- Move tasks between TODO and DONE lists
- Persistent storage (your tasks are saved and loaded automatically)

## Installation

1. Ensure you have [rust](https://www.rust-lang.org/tools/install) and [cargo](https://doc.rust-lang.org/cargo/commands/cargo-install.html) installed on your system.

2. Clone this repository:
   ```
   git clone https://github.com/Tramposo1312/terminal-todo.git
   cd terminal-todo
   ```

3. Build the project:
   ```
   cargo build --release
   ```

## Usage

Run the application using:

```
cargo run --release
```

## Key Bindings

| Key       | Function                                      |
|-----------|-----------------------------------------------|
| `i`       | Enter insert mode to add a new task           |
| `e`       | Edit the currently selected task              |
| `j`       | Move cursor down                              |
| `k`       | Move cursor up                                |
| `d`       | Delete the currently selected task            |
| `Enter`   | Move task between TODO and DONE lists         |
| `Tab`     | Switch focus between TODO and DONE lists      |
| `Esc`     | Exit insert or edit mode                      |
| `q`       | Quit the application                          |

When in insert or edit mode:
- `Enter` confirms your input
- `Esc` cancels the operation

## Data Persistence

Your progress is automatically saved to a file named `todo_list.txt` in the same directory as the application. This file is loaded when you start the application to ensure your tasks persist between sessions.
