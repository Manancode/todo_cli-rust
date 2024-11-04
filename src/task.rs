use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

// Define the Task structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub content: String,
    pub completed: bool,
}

// Functions for task operations
pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), io::Error> {
    let serialized = serde_json::to_string(&tasks)?;
    fs::write("tasks.json", serialized)
}

pub fn load_tasks() -> Result<Vec<Task>, io::Error> {
    match fs::read_to_string("tasks.json") {
        Ok(data) => {
            let tasks: Vec<Task> = serde_json::from_str(&data)?;
            Ok(tasks)
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e),
    }
}

pub fn add_task(tasks: &mut Vec<Task>, content: String) {
    let new_task = Task {
        id: tasks.len() as u32 + 1,
        content,
        completed: false,
    };
    tasks.push(new_task);
}

pub fn complete_task(tasks: &mut Vec<Task>, id: u32) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        println!("Task {} marked as complete!", id);
    } else {
        println!("Task with ID {} not found.", id);
    }
}

pub fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks found!");
        return;
    }

    println!("\nYour ToDo List:");
    println!("---------------");
    for task in tasks {
        let status = if task.completed { "✓" } else { " " };
        println!("{}. [{}] {}", task.id, status, task.content);
    }
}