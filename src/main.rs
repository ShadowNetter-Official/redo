use std::fs;
use serde::{Serialize, Deserialize};
use std::env;
use dirs; // new import for config path handling

fn strikethrough(text: &str) -> String {
    text.chars().map(|c| format!("{}\u{0336}", c)).collect()
}

// Helper function to get correct config path
fn get_data_path() -> std::path::PathBuf {
    let mut dir = dirs::config_dir().unwrap(); // usually ~/.config on Linux
    dir.push("redo");
    std::fs::create_dir_all(&dir).unwrap(); // make sure ~/.config/redo exists
    dir.push("tasks.json");
    dir
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new(description: String) -> Self {
        Self {
            description,
            done: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct List {
    todos: Vec<Task>
}

impl List {
    fn add(&mut self, description: String) {
        self.todos.push(Task::new(description));
    }

    fn new() -> Self {
        Self {
            todos: Vec::new()
        }
    }

    fn list(&self) {
        for (i, task) in self.todos.iter().enumerate() {
            let desc = if task.done {
                strikethrough(&task.description)
            } else {
                task.description.clone()
            };
            println!("{}. {}", i, desc);
        }
    }

    fn mark_done(&mut self, index: usize) {
        if let Some(task) = self.todos.get_mut(index) {
            task.done = true;
        }
    }

    fn save(&self) {
        let data = serde_json::to_string(&self).unwrap();
        fs::write(get_data_path(), data).unwrap();
    }

    fn load() -> Self {
        if let Ok(data) = fs::read_to_string(get_data_path()) {
            serde_json::from_str(&data).unwrap_or_else(|_| Self::new())
        } else {
            Self::new()
        }
    }

    fn remove(&mut self, index: usize) {
        if index < self.todos.len() {
            self.todos.remove(index);
        }
    }
}

fn help() {
    println!("redo | a fast todo list written in rust");
    println!("\nusage:");
    println!("redo <command> <task>\n");
    println!("redo add <task> -> add task to list");
    println!("redo show --> show tasks");
    println!("redo rem <task_number> -> remove task");
    println!("redo done <task_number> -> mark task as done\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut list = List::load();

    if args.len() < 2 {
        help();
        return; // prevent panic if no arguments
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                help();
            } else {
                list.add(args[2..].join(" "));
                list.save();
            }
        }
        "show" => {
            list.list();
        }
        "done" => {
            if args.len() < 3 {
                help();
            } else if let Ok(i) = args[2].parse::<usize>() {
                list.mark_done(i);
                list.save();
            }
        }
        "rem" => {
            if args.len() < 3 {
                help();
            } else if let Ok(i) = args[2].parse::<usize>() {
                list.remove(i);
                list.save();
            }
        }
        _ => help(),
    }
}
