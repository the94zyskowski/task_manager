mod tasks; // Import the tasks module for managing tasks

use std::env;
use std::path::PathBuf;

fn get_task_file_path() -> PathBuf {
    // Pobiera bieżący katalog roboczy
    let mut path = env::current_dir().expect("Failed to get current directory");

    path.push("tasks.json");
    path
}


fn main() {
    let file_path = get_task_file_path(); // Universal path to tasks.json
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: task_manager <command> [args]");
        return;
    }
    
    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: task_manager add <description>");
            } else {
                tasks::add_task(&file_path, args[2..].join(" ")).expect("Failed to add task");
            }
        }
        "list" => {
            tasks::display_tasks(&file_path).expect("Failed to display tasks");
        }
        "completed" => {
            if args.len() < 3 {
                eprintln!("Usage: task_manager completed <task_id>");
            } else if let Ok(id) = args[2].parse() {
                tasks::update_status(&file_path, id, true).expect("Failed to mark task as completed");
            }
        }
        "not_completed" => {
            if args.len() < 3 {
                eprintln!("Usage: task_manager not_completed <task_id>");
            } else if let Ok(id) = args[2].parse() {
                tasks::update_status(&file_path, id, false).expect("Failed to mark task as not completed");
            }
        }
        "delete" => {
            if args.len() < 3 {
                eprintln!("Usage: task_manager delete <task_id>")
            } else if let Ok(id) = args[2].parse() {
                tasks::delete_task(&file_path, id).expect("Failed to delete the task")
            }
        }
        _ => eprintln!("Unknown command"),
    }
}
