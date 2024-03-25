/**
    @author Leviathenn
**/

use std::{io, path::Path, process::exit};


struct Task {
    name: String,
    value: String,
    is_complete: bool,
}

fn main() {
    if Path::new("~/.todo/list.tdl").exists() {
        println!("Loading todo list...");
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
            let tasklist: Vec<Task>;
            for _i in 0..taskcount{
                let vname = rprompt::prompt_reply("name> ").unwrap();
                let value: String = rprompt::prompt_reply("value> ").unwrap();
                tasklist.append(&mut Task{
                    name: vname,
                    value: value,

                })
            }
        
        }else{
            println!("The input you inserted does not meet the required value.");
            exit(1);
        }

    }
}
