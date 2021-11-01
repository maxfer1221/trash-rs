use std::io::{self, BufReader, prelude::*, ErrorKind};
use std::fs::{File, read_dir};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
//use toml;
//use dirs_next;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub trash_dir: PathBuf,
}

impl Config {
    fn new(path: &PathBuf) -> Config {
        return Config { trash_dir: PathBuf::from(path) };
    }
}

pub fn create_config() -> Config {
    let (conf_loc, mut trash_loc) = resolve_dirs();
    let config: Config;

    match File::open(&conf_loc) {
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("Configuration file not found. Creating configuration file.");     
                let file: File = match File::create(&conf_loc) {
                    Err(e) => {
                        panic!("Unable to create configuration file: {:?}", e);
                    }
                    Ok(file) => file
                };

                trash_loc = verify_trash_loc(trash_loc);


                config = Config::new(&trash_loc);
                write_config(&file, &config);
            } other => { println!("Error while opening configuration file: {:?}", other); return Config::new(&trash_loc) }
        }
        Ok(file) => {
            let mut buf_reader = BufReader::new(&file);
            let mut contents = String::new();

            match buf_reader.read_to_string(&mut contents) {
                Err(error) => {
                    println!("Error reading configuration file: {:?}", error);
                    return Config::new(&trash_loc);
                },
                _ => {
                    let conversion = toml::from_str(&contents);
                    match conversion {
                        Err(e) => {
                            println!("Deserialization error: {:?}", e);
                            trash_loc = verify_trash_loc(trash_loc);
                            config = Config { trash_dir: trash_loc };
                            write_config(&file, &config);
                        },
                        Ok(conf) => { config = conf; }
                    }
                }
            }
        }
    }

    return config;
}

fn resolve_dirs() -> (PathBuf, PathBuf)  {
    let conf = match dirs_next::config_dir() {
        Some(mut buf) => {
            buf.push("trash-rs");
            buf.push("config");
            buf.set_extension("toml");
            buf
        }
        None => {
            panic!("OS \"config\" location unknown. Exiting.");
        }
    };
   
    let trash = match dirs_next::home_dir() {
        Some(mut buf) => {
            buf.push(".trash-rs");
            buf
        }
        None => {
            panic!("OS \"home\" location unknown. Exiting.");
        }
    };
    
    (conf, trash)
}

fn write_config(mut file: &File, config: &Config) {
    let toml_as_str: String = toml::to_string(config).unwrap();
    let bytes: &[u8] = (toml_as_str).as_bytes();
    match file.write(bytes) {
        Err(e) => {
            println!("Error writing to configuration file: {:?}", e);
        }
        _ => {}
    }
}

fn verify_trash_loc(mut trash_loc: PathBuf) -> PathBuf {    
    print!("Where would you like the trash directory to be? ");
    println!("Leave blank for the default locations:");
    println!("Lin: ~/.trash-rs");
    println!("\nWin: C:\\Users\\your_name\\.trash-rs");
    println!("Mac: /Users/your_name/.trash-rs");
          
    let mut desired_dir: String = String::new();
    match io::stdin().read_line(&mut desired_dir) {
        Err(e) => {
            println!("Error reading input: {:?}", e);
        }
        _ => {
            if !desired_dir.is_empty() {
                trash_loc = PathBuf::from(desired_dir);
            } 
        }
    }
    match read_dir(&trash_loc) {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => { panic!("Location given could not be found."); }
            _ => { println!("Error while reading directory given: {:?}", e); }
        } _ => {}
    }
    trash_loc
}
