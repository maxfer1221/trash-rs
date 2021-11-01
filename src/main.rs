//use std::{str::FromStr, env, fs::{File, self}};
//use std::{str::FromStr, env, fs};
use std::{str::FromStr, env, fs, io::ErrorKind};
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
    match fs::read_dir(&conf.trash_dir) {
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => match fs::create_dir(&conf.trash_dir) {
                    Err(e) => {
                        panic!("Problem creating trash directory: {:?}", e);
                    } _ => {}
                } other => {
                    panic!("Problem reading trash directory: {:?}", other);
                }
            }
        } _ => {}
    }

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
                println!("Usage: trash-rs delete FILE\n  or:  trash-rs delete FILES...\n");
                println!("Moves target FILE/S... to the trash directory as specified in the configuration file");
            } else {
                match delete::delete_files(rest, &conf.trash_dir) {
                    Err(e) => {
                        println!("{:?}", e);
                    }
                    _ => {}
                }
            }
        }
        Ok(Function::List) => {
            if help {
                println!("Usage: trash-rs list\n\nLists all files currently in the trash");
            } else {
                println!("List!");
            }
        }
        Ok(Function::Empty) => {
            if help {
                println!("Usage: trash-rs empty FILE\n  or:  trash-rs empty FILES...\n");
                println!("Permanently deletes specified files currently in the trash");
            } else {
                println!("Empty!");
            }
        }
        Ok(Function::ChangeDir(_)) => {
            if help {
                println!("Usage: trash-rs change-dir DIR\n\nChanges the target directory for trashed items");
            } else {
                println!("Change dir!");
            }
        }
        Ok(Function::Restore(_)) => {
            if help {
                println!("Usage: trash-rs restore FILE\n  or:  trash-rs restore FILES...\n");
                println!("Restores files from the trashcan to their original directories (if possible)");
            } else {
                println!("Restore!");
            }
        }
        Err(()) => {
            if help {
                println!("manpage");
            } else {
                println!("{} is not a function. 'trash-rs --help' for more information.", args[1]);
            }
            let attr = fs::metadata("/home/maximo/Documents/trash-rs/src/main.rs").unwrap();
            println!("{:?}", attr);
        }
    }

}
