//use std::{str::FromStr, env, fs::{File, self}};
//use std::{str::FromStr, env, fs};
use std::{str::FromStr, env};
mod config;
//use delete;

#[derive(Debug, PartialEq)]
enum Function {
    Delete,
    List,
    Empty,
    ChangeDir(Option<String>),
    Restore(Option<Box<[String]>>),
}

impl FromStr for Function {
    type Err = ();

    fn from_str(input: &str) -> Result<Function, Self::Err> {
        match input {
            "delete"     => Ok(Function::Delete),
            "list"       => Ok(Function::List),
            "empty"      => Ok(Function::Empty),
            "change-dir" => Ok(Function::ChangeDir(None)),
            "Restore"    => Ok(Function::Restore(None)),
            _            => Err(()),
        }
    }
}

fn main() {
    
    let args: &[String] = &env::args().collect::<Vec<String>>();
    let function = Function::from_str(&args[1]);
    
    match function {
        Ok(Function::Delete) => {
            println!("Delete!");       
        }
        Ok(Function::List) => {
            println!("List!");
        }
        Ok(Function::Empty) => {
            println!("Empty!");
        }
        Ok(Function::ChangeDir(_)) => {
            println!("Change dir!");
        }
        Ok(Function::Restore(_)) => {
            println!("Restore!");
        }
        Err(()) => {
            println!("{} is not a function.", args[1]);
        }
    }

    config::create_config();
}
