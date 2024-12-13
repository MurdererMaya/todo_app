use clap::{App, Arg, SubCommand};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)] 
struct TodoItem {
    id: u32,
    description: String,
}

#[derive(Serialize, Deserialize)] 
struct TodoList {
    items: HashMap<u32, TodoItem>,
    next_id: u32,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            items: HashMap::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, description: String) {
        let id = self.next_id;
        self.items.insert(id, TodoItem { id, description });
        self.next_id += 1;
        println!("Added task with ID: {}", id);
    }

    fn view(&self) {
        for item in self.items.values() {
            println!("{}: {}", item.id, item.description);
        }
    }

    fn remove(&mut self, id: u32) {
        if self.items.remove(&id).is_some() {
            println!("Removed task with ID: {}", id);
        } else {
            println!("Task with ID {} not found.", id);
        }
    }

    fn save(&self) -> io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("todo_list.json")?;
        serde_json::to_writer(file, &self)?; 
        Ok(())
    }

    fn load() -> io::Result<Self> {
        let mut file = match File::open("todo_list.json") {
            Ok(file) => file,
            Err(_) => return Ok(TodoList::new()),
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let todo_list = serde_json::from_str(&contents)?;
        Ok(todo_list)
    }
}

fn main() {
    let matches = App::new("Todo App")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Manages a simple to-do list")
        .subcommand(SubCommand::with_name("add")
            .about("Adds a new task")
            .arg(Arg::with_name("DESCRIPTION")
                .help("The description of the task")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("view")
            .about("Views all tasks"))
        .subcommand(SubCommand::with_name("remove")
            .about("Removes a task")
            .arg(Arg::with_name("ID")
                .help("The ID of the task")
                .required(true)
                .index(1)))
        .get_matches();

    let mut todo_list = TodoList::load().expect("Failed to load tasks");

    if let Some(matches) = matches.subcommand_matches("add") {
        if let Some(description) = matches.value_of("DESCRIPTION") {
            todo_list.add(description.to_string());
        }
    } else if let Some(_) = matches.subcommand_matches("view") {
        todo_list.view();
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        if let Some(id) = matches.value_of("ID") {
            if let Ok(id) = id.parse::<u32>() {
                todo_list.remove(id);
            } else {
                println!("Invalid ID.");
            }
        }
    }

    todo_list.save().expect("Failed to save tasks"); 
}
