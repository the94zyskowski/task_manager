# task_manager
A simple task management application written in Rust, designed to work on both Windows and Linux. This program enables users to manage a list of tasks, saving data in a tasks.json file. The file is stored in the task_manager folder within the directory from which the program is run, making the application portable across different systems.
## Features
- Add a Task: Create a new task with a unique ID and description.
- List All Tasks: Display all tasks with their status (completed or not completed).
- Mark Task as Completed: Set a task's status to "completed".
- Mark Task as Not Completed: Set a task's status back to "not completed".
- Delete a Task: Remove a specific task by ID.
- Cross-Platform Compatibility: Works seamlessly on both Windows and Linux.
## Example and usage
Make sure you have Rust installed on your system. To run the program, navigate to the project directory in your terminal and use the following commands:
```sh
$ cargo run -- add "Learn Rust"
$ cargo run -- list
[1] Learn Rust - Not completed
$ cargo run -- completed 1
$ cargo run -- list
[1] Learn Rust - Completed
$ cargo run -- delete 1
$ cargo run -- list
No entries
```
## Installation and Setup
Clone the repository:
```sh
git clone https://github.com/your_username/task_manager.git
cd task_manager
```
Run the project:
```sh
cargo run -- <command> [arguments]
```
## Requirements
- Rust (for compilation and running the application)
- To customize the GitHub link in the "Clone the repository" section, replace https://github.com/your_username/task_manager.git with the actual link to your GitHub repository.
