// src/main.rs

// #[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }
}


use std::io::{self, Write};

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("Enter a command (add, list, complete, exit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        } else if input.starts_with("add ") {
            let description = input[4..].to_string();
            tasks.push(Task::new(description));
            println!("Task added.");
        } else if input == "list" {
            for (i, task) in tasks.iter().enumerate() {
                println!("{}: {} [{}]", i, task.description, if task.completed { "x" } else { " " });
            }
        } else if input.starts_with("complete ") {
            if let Ok(index) = input[9..].parse::<usize>() {
                if index < tasks.len() {
                    tasks[index].completed = true;
                    println!("Task {} marked as completed.", index);
                } else {
                    println!("Invalid task index.");
                }
            } else {
                println!("Please provide a valid task index.");
            }
        } else {
            println!("Invalid command.");
        }
    }
}
