use std::fs;
use std::io::{self, Write};
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
    let mut tasks = load_tasks();

    loop {
        print!("todo> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input.");
            continue;
        }
        let input = input.trim();
        if input.eq_ignore_ascii_case("stop") || input.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }
        let args: Vec<&str> = input.split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        match args[0] {
            "add" => {
                if args.len() < 2 {
                    println!("Please provide a task to add.");
                } else {
                    let task = args[1..].join(" ");
                    add_task(task, &mut tasks);
                    println!("Task added.");
                }
            }
            "finish" => {
                if args.len() < 2 {
                    println!("Please provide the index of the task to finish.");
                } else {
                    match args[1].parse::<usize>() {
                        Ok(idx) if idx > 0 && idx <= tasks.len() => {
                            let removed = tasks.remove(idx - 1);
                            let data = serde_json::to_string_pretty(&tasks).expect("Failed to serialize tasks");
                            fs::write("todo.json", data).expect("Unable to write file");
                            println!("Finished task: {}", removed);
                        }
                        _ => println!("Invalid index."),
                    }
                }
            }
            "print" => {
                println!("Tasks:");
                for (i, task) in tasks.iter().enumerate() {
                    println!("{}: {}", i + 1, task);
                }
            }
            _ => {
                println!("Unknown command: {}", args[0]);
            }
        }
    }
}

