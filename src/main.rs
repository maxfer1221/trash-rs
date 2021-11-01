//use std::{str::FromStr, env, fs::{File, self}};
//use std::{str::FromStr, env, fs};
use std::{str::FromStr, env};
mod config;
mod delete;


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
    let conf: config::Config = config::create_config();
    
    let args: &[String] = &env::args().collect::<Vec<String>>(); 
    let flags = args.iter().filter(|a| a.starts_with('-'))
        .cloned().collect::<Vec<String>>();
    let rest  = args.iter().filter(|a| !a.starts_with('-'))
        .cloned().collect::<Vec<String>>();
    let help: bool = flags.iter().any(|f| f == "-h" || f == "--help");
    let function = Function::from_str(&args[1]);

    match function {
        Ok(Function::Delete) => {
            if help {
                println!("Usage: trash-rs delete FILE\n  or:  trash-rs delete FILES...");
            } else {
                let res = delete::delete_files(rest, &conf.trash_dir);
                match res {
                    Err(e) => {
                        println!("{:?}", e);
                    }
                    _ => {}
                }
            }
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
            if help {
                println!("help msg");
            } else {
                println!("{} is not a function. do 'trash-rs --help' for more information.", args[1]);
            }
        }
    }

}
