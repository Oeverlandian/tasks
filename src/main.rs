use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use text_io::read;
use serde_json;
use std::fs;

#[derive(Parser)]
#[clap(name = "tasks")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    List,
    Add {
        #[clap(long)]
        completed: bool,
    },
    Complete,
    Remove
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    completed: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut tasks: Vec<Task> = Vec::new();

    match cli.command {
        Command::List => {
            list();
        }
        Command::Add { completed } => {
            if completed {
                add_completed(&mut tasks);
            } else {
                add(&mut tasks);
            }
        }
        Command::Complete => {
            complete();
        }
        Command::Remove => {
            remove();
        }
    }
}

fn list() {
    let json_data = fs::read_to_string("tasks.json").expect("Unable to read file");
    let tasks: Vec<Task> = serde_json::from_str(&json_data).unwrap();

    for (index, task) in tasks.iter().enumerate() {
        println!("{}. {} - {}", index + 1, task.name, if task.completed { "Completed" } else { "Incomplete" });
    }
}

fn add(tasks: &mut Vec<Task>) {
    print!("Enter the name of the task: ");
    let name: String = read!();
    let task = Task { name, completed: false };

    tasks.push(task);
    println!("Task added.");
    let json = serde_json::to_string(&tasks).unwrap();
    fs::write("tasks.json", json).expect("Unable to write file");
}

fn add_completed(tasks: &mut Vec<Task>) {
    print!("Enter the name of the task: ");
    let name: String = read!();
    let task = Task { name, completed: true };

    tasks.push(task);
    println!("Task added.");
    let json = serde_json::to_string(&tasks).unwrap();
    fs::write("tasks.json", json).expect("Unable to write file");
}

fn complete() {
    let json_data = fs::read_to_string("tasks.json").expect("Unable to read file");
    let mut tasks: Vec<Task> = serde_json::from_str(&json_data).unwrap();

    list();

    print!("Enter the number of the task to complete: ");
    let task_number: usize = read!();

    if task_number > 0 && task_number <= tasks.len() {
        tasks[task_number - 1].completed = true;
        println!("Task {} marked as complete.", task_number);
        let json = serde_json::to_string(&tasks).unwrap();
        fs::write("tasks.json", json).expect("Unable to write file");
    } else {
        println!("Invalid task number.");
    }
}

fn remove() {
    let json_data = fs::read_to_string("tasks.json").expect("Unable to read file");
    let mut tasks: Vec<Task> = serde_json::from_str(&json_data).unwrap();

    list();

    print!("Enter the number of the task to delete: ");
    let task_number: usize = read!();

    if task_number > 0 && task_number <= tasks.len() {
        tasks.remove(task_number - 1);
        println!("Task {} deleted.", task_number);
        let json = serde_json::to_string(&tasks).unwrap();
        fs::write("tasks.json", json).expect("Unable to write file");
    } else {
        println!("Invalid task number.");
    }
}