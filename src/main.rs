//use std::{str::FromStr, env, fs::{File, self}};
//use std::{str::FromStr, env, fs};
use std::{str::FromStr, env, fs, io::ErrorKind};
mod config;
mod delete;
mod list;
mod trash;

#[derive(Debug, PartialEq)]
enum Function {
    Delete,
    List,
    Empty,
    ChangeDir(Option<String>),
    Restore(Option<Box<[String]>>),
    Clean,
}

impl FromStr for Function {
    type Err = ();

    fn from_str(input: &str) -> Result<Function, Self::Err> {
        match input {
            "delete"     => Ok(Function::Delete),
            "list"       => Ok(Function::List),
            "empty"      => Ok(Function::Empty),
            "change-dir" => Ok(Function::ChangeDir(None)),
            "restore"    => Ok(Function::Restore(None)),
            "clean"     => Ok(Function::Clean),
            _            => Err(()),
        }
    }
}

fn main() {
    let args: &[String] = &env::args().collect::<Vec<String>>(); 
    let flags = args.iter().filter(|a| a.starts_with('-'))
        .cloned().collect::<Vec<String>>();
    let rest  = args.iter().filter(|a| !a.starts_with('-'))
        .cloned().collect::<Vec<String>>();
    let help: bool = flags.iter().any(|f| f == "-h" || f == "--help");

    let conf: config::Config;

    match Function::from_str(&args[1]) {
        Ok(Function::Delete) => {
            if help {
                println!("Usage: trash-rs delete FILE\n  or:  trash-rs delete FILES...\n");
                println!("Moves target FILE/S... to the trash directory as specified in the configuration file");
            } else {
                conf = fetch_config();
                match delete::delete_files(rest, &conf) {
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
                conf = fetch_config();
                match list::list_objects(rest, &conf) {
                    Err(e) => {
                        println!("{:?}", e);
                    }
                    _ => {}
                }
            }
        }
        Ok(Function::Empty) => {
            if help {
                println!("Usage: trash-rs empty FILE\n  or:  trash-rs empty FILES...\n");
                println!("Permanently deletes specified files currently in the trash");
            } else {
                // conf = fetch_config();
                println!("Empty!");
            }
        }
        Ok(Function::ChangeDir(_)) => {
            if help {
                println!("Usage: trash-rs change-dir DIR\n\nChanges the target directory for trashed items");
            } else {
                // conf = fetch_config();
                println!("Change dir!");
            }
        }
        Ok(Function::Restore(_)) => {
            if help {
                println!("Usage: trash-rs restore FILE\n  or:  trash-rs restore FILES...\n");
                println!("Restores files from the trashcan to their original directories (if possible)");
            } else {
                // conf = fetch_config();
                println!("Restore!");
            }
        },
        Ok(Function::Clean) => {
            let help_str: String = format!("{}\n{}\n{}\n{}",
                "Usage: trash-rs clean FLAGS\n\nCreates (overwrites) configuration file or trash directories",
                "FLAGS:\n  -c        Recreate configuration file",
                "  -d        Recreate trash directories",
                "  -cd  -dc  Recreate configuration file and trash directories");
            if help {
                println!("{}", help_str);
            } else {
                println!("{:?}", flags);
                if flags.iter().any(|f| f.contains("c")) {
                    println!("here");
                    config::create_config_file(&config::find_conf());
                } else if flags.iter().any(|f| f.contains("d")) {
                    config::create_master_dir();
                } else {
                    println!("{}", help_str);
                }
            }
        },
        Err(()) => {
            if help {
                println!("manpage");
            } else {
                println!("'{}' is not a function. Do 'trash-rs --help' for more information", args[1]);
            }
        }
    }
}

fn fetch_config() -> config::Config {
    let conf: config::Config = config::fetch_config();
    match fs::read_dir(&conf.dirs.trash_dir) {
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => match fs::create_dir(&conf.dirs.trash_dir) {
                    Err(e) => {
                        println!("Problem creating trash directory: {:?}", e);
                        std::process::exit(1);
                    } _ => {}
                } other => {
                    println!("Problem reading trash directory: {:?}", other);
                    std::process::exit(1);
                }
            }
        } _ => {}
    }
    conf
}
