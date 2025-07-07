use std::env;
use std::fs;
use std::path::Path;

fn load_tasks() -> Vec<String> {
    let path = "todo.json";
    if !Path::new(path).exists() {
        return Vec::new();
    }

    let data = fs::read_to_string(path).unwrap_or_else(|_| String::from("[]"));
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

fn add_task(task: String, tasks: &mut Vec<String>) {
    tasks.push(task);
    let data = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write("todo.json", data).expect("Unable to write file");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut tasks = load_tasks();

    if args.len() < 2 {
        println!("Usage: todo-cli <add|delete|print> [task]");
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Please provide a task to add.");
            } else {
                add_task(args[2].clone(), &mut tasks);
                println!("Task added: {}", args[2]);
            }
        }
         "finish" => {
            println!("Delete not implemented yet.");
        }
        "print" => {
            println!("Tasks:");
            for (i, task) in tasks.iter().enumerate() {
                println!("{}: {}", i + 1, task);
            }
        }
        _ => {
            println!("Unknown command: {}", args[1]);
        }
    }
}

