use std::{str::FromStr, env, fs, path::PathBuf, io::{Error, ErrorKind}};
mod config;
mod delete;
mod trash;
mod empty;
mod list;

macro_rules! panic_ {
    ( $s:expr, $e:expr ) => {
        println!($s, $e);
        std::process::exit(1);
    }
}

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
            "clean"      => Ok(Function::Clean),
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

    if args.len() == 1 {
        println!("Please provide a function. Do 'trash-rs -h' for more information");
        std::process::exit(1);
    }

    match Function::from_str(&args[1]) {
        Ok(Function::Delete) => {
            if help {
                println!("Usage: trash-rs delete FILE\n  or:  trash-rs delete FILES...\n");
                println!("Moves target FILE/S... to the trash directory as specified in the configuration file");
            } else {
                conf = match fetch_config() {
                    Ok(c) => c,
                    Err(e) => {
                        panic_!("Error fetching configuration file: {:?}", e);
                    }
                };
                match delete::delete_files(rest, &conf) {
                    Err(e) => {
                        panic_!("Error deleting files: {:?}", e);
                    }
                    _ => {}
                }
            }
        }
        Ok(Function::List) => {
            if help {
                println!("Usage: trash-rs list\n\nLists all files currently in the trash");
            } else {
                conf = match fetch_config() {
                    Ok(c) => c,
                    Err(e) => {
                        panic_!("Error fetching configuration file: {:?}", e);
                    }
                };
                match list::list_objects(rest, &conf) {
                    Err(e) => {
                        panic_!("Error listing files: {:?}", e);
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
                conf = match fetch_config() {
                    Ok(c) => c,
                    Err(e) => {
                        panic_!("Error fetching configuration file: {:?}", e);
                    }
                };
                match empty::perm_delete_files(rest, &conf) {
                    Err(e) => {
                        panic_!("Error emptying trash directory: {:?}", e);
                    }, _ => {}
                };
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
                if flags.iter().any(|f| f.contains("c") || f.contains("d")) {
                    if flags.iter().any(|f| f.contains("c")) {
                        let cd: PathBuf = match config::find_conf() {
                            Ok(d) => d,
                            Err(e) => {
                                println!("Error finding configuration directory: {:?}", e);
                                std::process::exit(1);
                            }
                        };
                        match config::create_config_file(&cd) {
                            Ok(c) => c,
                            Err(e) => {
                                println!("Error creating configuration file: {:?}", e);
                                std::process::exit(1);
                            }
                        };
                    }
                    if flags.iter().any(|f| f.contains("d")) {
                        if let Err(e) = config::create_master_dir() {
                            println!("Error creating directories: {:?}", e);
                        };
                    }
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

fn fetch_config() -> Result<config::Config, Error> {
    let conf: config::Config = config::fetch_config()?;
    Ok(conf)
}
