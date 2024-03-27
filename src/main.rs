/**
 @author Leviathenn
 **/
 
 use std::{io, process::exit};
 use std::fs::File;
 use std::io::Write;
 use std::io::prelude::*;
 use std::path::Path;
 use std::fmt::format;
use std::fs::read_to_string;
struct Task {
    name: String,
    is_complete: bool,
}
struct TaskVector {
    tasks: Vec<Task>,
}

impl TaskVector {
    // Create a new empty TaskVector
    fn new() -> Self {
        TaskVector { tasks: Vec::new() }
    }

    // Append a task to the TaskVector
    fn append(&mut self, name: String, is_complete: bool) {
        let task = Task {
            name,
            is_complete,
        };
        self.tasks.push(task);
    }

    // Get the length of the TaskVector
    fn len(&self) -> usize {
        self.tasks.len()
    }

    // Check if the TaskVector is empty
    // Get a task by index
    fn get(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn main() {
    if Path::new("./.tasks").exists() {
        if let Ok(lines) = read_lines(".tasks") {
            // Consumes the iterator, returns an (Optional) String
            for line in lines.flatten() {
                for text in line.split("[%20]"){
                    
                }
            }
        }
    }else{
        println!("Welcome to the todo list! Lets begin with some task you want to complete.");
        
        println!("Press 1 to continue.");
        let mut numc = String::new();
        io::stdin().read_line(&mut numc).expect("You did not input a value.");
        
        let numc: u32 = numc.trim().parse().expect("The input you inserted does not equal a numerical value.");
        if numc == 1 {
            println!("Great! Now were going to ask some questions.");
            println!("How many task do you want to complete?");
            let taskcount:u32 = rprompt::prompt_reply("answer> ").unwrap().trim().parse().expect("You did not enter a numerical value.");
            let mut taskvector = TaskVector::new();
            for _i in 0..taskcount{
                let vname = rprompt::prompt_reply("name> ").unwrap();
                let value: bool = rprompt::prompt_reply("isComplete(1 for true 2 for false)> ").unwrap().parse().expect("Didn't get 1 or 2.");
                taskvector.append(vname, value);
            }
            let mut taskString = String::new();
            for i in 0..taskvector.len(){
                let task: &Task = taskvector.get(i).unwrap();
                println!("Task{}: Name: {}, isComplete: {}",i,task.name,task.is_complete);
                taskString = format!("\n{}:{}",task.name.replace(" ","[%20]"),task.is_complete);
                
                
            }
            println!("Saving file.");
            let path = Path::new(".tasks");
            let display = path.display();

            // Open a file in write-only mode, returns `io::Result<File>`
            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", display, why),
                Ok(file) => file,
            };
        
            // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
            match file.write_all(taskString.as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why),
                Ok(_) => println!("successfully wrote to {}", display),
            }
        }else{
            println!("The input you inserted does not meet the required value.");
            exit(1);
        }

    }
}
