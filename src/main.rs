use clap::{Parser, Subcommand};
use serde::{self, Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        name: String
    },

    List,

    Delete {
        name: String
    },

    Rename {
        name: String,
        new_name: String
    },

    Update {
        name: String,
        state: bool
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    name: String,
    done: bool
}

struct TaskList {
    tasks: Vec<Task>
}

impl TaskList {
    fn load() -> Self {
        let home_dir = std::env::var("HOME").unwrap();
        let file_path = format!("{home_dir}/.tasks.json");
        
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(_) => return TaskList { tasks: Vec::new() }
        };

        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_err() {
            return TaskList { tasks: Vec::new() };
        }

        let tasks = serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
        TaskList { tasks }
    }

    fn save(&self) {
        let home_dir = std::env::var("HOME").unwrap();
        let file_path = format!("{home_dir}/.tasks.json");
        let mut file = File::create(file_path).expect("Failed to create file");

        let json_string = serde_json::to_string_pretty(&self.tasks).expect("Failed to write file");

        file.write_all(json_string.as_bytes()).expect("Failed to save tasks")
    }

    fn find_index(&self, name: &String) -> Result<usize, &'static str> {
        match self.tasks.iter().position(|t| &t.name == name) {
            Some(pos) => Ok(pos),
            None => Err("Task was not found")
        }
    }

    fn add(&mut self, name: &String) -> Result<(), &'static str> {
        if self.find_index(name).is_ok() {
            return Err("Another task is using that name already");
        };

        let new_task = Task { name: name.clone(), done: false};

        self.tasks.push(new_task);
        self.save();
        Ok(())
    }

    fn list(&self) {
        for task in &self.tasks {
            println!("Name: {}, State: {}", task.name, if task.done == true {"done"} else {"not done"})
        }
    }

    fn delete(&mut self, name: &String) -> Result<(), &'static str> {
        match self.find_index(name) {
            Ok(idx) => { 
                self.tasks.remove(idx);
                self.save();
                return Ok(());
            },
            Err(e) => panic!("{e}")
        }
    }
}

impl Task {
    fn update(&mut self, state: bool) {
        self.done = state;
    }

    fn rename(&mut self, name: &String) {
        self.name = name.clone();
    }
}


fn main() {
    let args = Cli::parse();

    let mut tasks = TaskList::load();

    match args.command {
        Commands::Add { name } => {
            match tasks.add(&name) {
                Ok(_) => println!("Saved task {name}"),
                Err(e) => panic!("{e}")
            }
        },
        Commands::List => {
            println!("Here are all the tasks: \n");
            tasks.list()
        },
        Commands::Delete { name } => {
            match tasks.delete(&name) {
                Ok(_) => println!("Deleted task {name}"),
                Err(e) => panic!("{e}")
            }
        },
        Commands::Rename { name, new_name } => {
            let index = tasks.find_index(&name).unwrap();
            let task = &mut tasks.tasks[index];

            task.rename(&new_name);

            tasks.save();

            println!("Renamed {name} to {new_name}")
        },
        Commands::Update { name, state } => {
            let index = tasks.find_index(&name).unwrap();
            let task = &mut tasks.tasks[index];
            
            task.update(state);

            tasks.save();

            println!("{name} is now {}", if state == true { "Done" } else { "Not done" })
        }
    }
}
