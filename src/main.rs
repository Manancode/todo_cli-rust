mod task;

use clap::{App, Arg};
use task::{add_task, complete_task, list_tasks, load_tasks, save_tasks};

fn main() {
    // Set up command line interface
    let matches = App::new("ToDo CLI")
        .version("1.0")
        .author("Your Name")
        .about("Manage your tasks")
        .arg(
            Arg::new("add")
                .short('a')
                .long("add")
                .value_name("TASK")
                .help("Adds a task to your to-do list")
                .takes_value(true),
        )
        .arg(
            Arg::new("complete")
                .short('c')
                .long("complete")
                .value_name("ID")
                .help("Marks a task as complete")
                .takes_value(true),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("Lists all tasks"),
        )
        .get_matches();

    // Load existing tasks
    let mut tasks = match load_tasks() {
        Ok(tasks) => tasks,
        Err(e) => {
            println!("Error loading tasks: {}", e);
            return;
        }
    };

    // Handle different commands
    if let Some(task_content) = matches.value_of("add") {
        add_task(&mut tasks, task_content.to_string());
        println!("Task added successfully!");
    } else if let Some(task_id) = matches.value_of("complete") {
        match task_id.parse::<u32>() {
            Ok(id) => complete_task(&mut tasks, id),
            Err(_) => println!("Invalid task ID"),
        }
    } else if matches.is_present("list") {
        list_tasks(&tasks);
    } else {
        println!("No command specified. Use --help for usage information.");
    }

    // Save tasks after any modification
    if let Err(e) = save_tasks(&tasks) {
        println!("Error saving tasks: {}", e);
    }
}