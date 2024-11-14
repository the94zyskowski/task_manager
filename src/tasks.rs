use serde::{Deserialize, Serialize}; // Import for JSON serialization and deserialization

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,            // Unique identifier for the task
    pub description: String,   // Description of the task
    pub completed: bool,       // Completion status
}

impl Task {
    // Creates a new task with a unique ID and description
    pub fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
        }
    }
}

use std::fs::OpenOptions;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;
use serde_json;

// Loads tasks from the specified JSON file
pub fn load_tasks(path: &Path) -> io::Result<Vec<Task>> {
    eprintln!("Attempting to open or create file at path: {:?}", path.display());

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .map_err(|e| {
            eprintln!("Error opening or creating file: {:?}", e);
            e
        })?;

    // Uses a buffered reader to handle JSON content in the file
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader).unwrap_or_else(|_| vec![]); // Returns an empty list if JSON is invalid or file is empty
    Ok(tasks)
}

// Saves the list of tasks to the specified JSON file
pub fn save_tasks(path: &Path, tasks: &Vec<Task>) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Empties the file before writing new data
        .open(path)?;

    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, tasks)?; // Serializes tasks to JSON format
    Ok(())
}

// Adds a new task to the list and saves it to the JSON file
pub fn add_task(path: &Path, description: String) -> io::Result<()> {
    let mut tasks = load_tasks(path)?; // Loads existing tasks
    let id = tasks.len() + 1; // Generates a new unique ID
    let new_task = Task::new(id, description);
    tasks.push(new_task); // Adds the new task to the list
    save_tasks(path, &tasks) // Saves the updated list back to the file
}

// Displays all tasks with their status
pub fn display_tasks(path: &Path) -> io::Result<()> {
    let tasks = load_tasks(path)?; // Loads tasks from file
    if tasks.is_empty() {
        println!("No entries");
    } else {
        for task in tasks {
            println!(
                "[{}] {} - {}",
                task.id,
                task.description,
                if task.completed { "Completed" } else { "Not completed" }
            );
        }
    }
    Ok(())
}

// Change status of the task
pub fn update_status(path: &Path, id: usize, is_completed: bool) -> io::Result<()> {
    let mut tasks = load_tasks(path)?; // Loads tasks from file as mutable
    for task in &mut tasks {
        if task.id == id {
            task.completed = is_completed; // Sets completed status based on is_completed
        }
    }
    save_tasks(path, &tasks) // Saves updated tasks list to file
}

// Delete task
// Delete task
pub fn delete_task(path: &Path, id: usize) -> io::Result<()> {
    let mut tasks = load_tasks(path)?; // Loads tasks from file as mutable

    // Remove the task with the specified id
    tasks.retain(|task| task.id != id); // Keeps only tasks that do not match the given id

    save_tasks(path, &tasks) // Saves updated tasks list to file
}
