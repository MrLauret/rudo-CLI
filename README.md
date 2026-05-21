Usage Syntax

Since the binary runs through Cargo during development, use the -- flag to separate Cargo arguments from the CLI arguments:
1. Add a New Task

New tasks are inserted at the end of the list and default to an uncompleted state. Duplicate names are rejected safely.
Bash

cargo run -- add "Finish Rust project"

2. List All Tasks

Prints out all your tracked tasks along with their real-time state.
Bash

cargo run -- list

3. Update Task Status

Mark a task as completed (true) or incomplete (false).
Bash

cargo run -- update "Finish Rust project" true

4. Rename a Task

Fix typos or update task descriptions easily.
Bash

cargo run -- rename "Finish Rust project" "Master the Rust Borrow Checker"

5. Delete a Task

Permanently wipe a task from your tracking file.
Bash

cargo run -- delete "Master the Rust Borrow Checker"

Technical Stack & Learnings

This project serves as an implementation template for mastering core Rust paradigms:

    clap (v4): Utilizes the modern Parser and Subcommand derive macros to compile command routing at compile time.

    Serialization: Leverages serde and serde_json to bridge the gap between memory-allocated vectors (Vec<Task>) and flat text files.

    Memory Safety: Handles clean mutable borrows (&mut) inside the application matrix, ensuring zero data races and memory optimization without explicit memory cloning during iterations.

Data Schema

Your tasks are organized inside a Tasks.json root file with the following structured schema:
JSON

[
  {
    "name": "Buy milk",
    "done": false
  }
]

"""

with open("README.md", "w") as f:
f.write(readme_content)

Your `README.md` file is ready.

[file-tag: code-generated-file-0-1779295115082596835]

Here is a preview of the clean, professional documentation generated for your project:

***

# Task Master CLI

A blazing fast, reliable, and persistent command-line To-Do list manager built from scratch in Rust. This application utilizes `clap` for structured subcommand parsing and `serde` for automatic JSON data persistence.

## Features

- **Robust Architecture**: Built with idiomatic Rust, utilizing data encapsulation via structured implementation (`impl`) blocks.
- **Subcommand Layout**: Zero clunky flags. Run clean commands like `tasks add "Buy milk"`.
- **Automatic JSON Persistence**: Tasks are saved locally into a beautifully formatted `Tasks.json` file on every modification.
- **Full CRUD Support**: Create, Read, Update, and Delete tasks seamlessly from your terminal.

---

## Installation

Ensure you have Rust and `cargo` installed on your system. If not, get it from [rustup.rs](https://rustup.rs/).

1. Clone or navigate to your project directory.
2. Build the project using Cargo:
   ```bash
   cargo build --release

Usage Syntax

Since the binary runs through Cargo during development, use the -- flag to separate Cargo arguments from the CLI arguments:
1. Add a New Task
Bash

cargo run -- add "Finish Rust project"

2. List All Tasks
Bash

cargo run -- list

3. Update Task Status
Bash

cargo run -- update "Finish Rust project" true

4. Rename a Task
Bash

cargo run -- rename "Finish Rust project" "Master the Rust Borrow Checker"

5. Delete a Task
Bash

cargo run -- delete "Master the Rust Borrow Checker"

Technical Stack & Learnings

This project serves as an implementation template for mastering core Rust paradigms:

    clap (v4): Utilizes the modern Parser and Subcommand derive macros to compile command routing at compile time.

    Serialization: Leverages serde and serde_json to bridge the gap between memory-allocated vectors (Vec<Task>) and flat text files.

    Memory Safety: Handles clean mutable borrows (&mut) inside the application matrix, ensuring zero data races and memory optimization without explicit memory cloning.