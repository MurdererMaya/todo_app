# Todo App

A simple command-line application to manage a to-do list. This app allows you to add, view, and remove tasks, and save/load the to-do list from a JSON file.

## Features

- **Add Tasks**: Add new tasks to your to-do list with a description.
- **View Tasks**: View all tasks in your to-do list.
- **Remove Tasks**: Remove tasks from your to-do list by their ID.
- **Persistent Storage**: Save and load your to-do list from a JSON file.

## Installation

To set up and run this Todo App, follow these steps:

1. **Clone the repository**:

    ```sh
    git clone https://github.com/murderermaya/todo_app.git
    cd todo_app
    ```

2. **Install Rust** (if not already installed):

    Follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

3. **Build the project**:

    ```sh
    cargo build
    ```

4. **Run the project**:

    ```sh
    cargo run -- <command> [args]
    ```

## Usage

### Add a Task

To add a new task to your to-do list:

```sh
cargo run -- add "Your task description here"
```
### Remove a Task

To remove a task from your to-do list:

```sh
cargo run -- remove [ID]
```
### View Task List

To view the tasks in your to-do list:

```sh
cargo run -- view
```
