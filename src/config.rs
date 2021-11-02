use std::io::{self, BufReader, prelude::*, ErrorKind};
use std::fs::{self, File, read_dir};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
//use toml;
//use dirs_next;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub master_dir: PathBuf,
    pub trash_dir: PathBuf,
    pub trash_info: PathBuf,
}

impl Config {
    fn new(master_dir: &PathBuf) -> Config {
        let mut tp: PathBuf = master_dir.clone();
        let mut ti: PathBuf = master_dir.clone();
        tp.push("files");
        ti.push("info");
        Config {
            master_dir: master_dir.clone(),
            trash_dir: tp,
            trash_info: ti,
        }
    }
}

pub fn fetch_config() -> Config {
    let conf_loc: PathBuf  = find_conf();

    match File::open(&conf_loc) {
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                let (file, config) = create_config_file(&conf_loc);
                write_config(&file, &config);
                config
            } other => {
                println!("Error while opening configuration file: {:?}", other);
                println!("Create new configuration file? (y/n)");

                if parse_yn() {
                    let (file, config) = create_config_file(&conf_loc);
                    write_config(&file, &config);
                    config
                } else {
                    println!("Exiting");
                    std::process::exit(1);
                }
            }
        },
        Ok(file) => {
            let mut buf_reader = BufReader::new(&file);
            let mut contents = String::new();

            match buf_reader.read_to_string(&mut contents) {
                Err(error) => {
                    println!("Error reading configuration file: {:?}", error);
                    println!("Exiting");
                    std::process::exit(1);
                },
                _ => {},
            }

            if let Ok(config) = toml::from_str(&contents) {
                config
            } else {
                println!("Deserialization error. Do 'trash-rs clean' to reset configuration file");
                std::process::exit(1);
            }
        }
    }
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

pub fn find_conf() -> PathBuf {
    match dirs_next::config_dir() {
        Some(mut buf) => {
            buf.push("trash-rs");
            buf.push("config");
            buf.set_extension("toml");
            buf
        }
        None => {
            println!("OS \"config\" location unknown. Exiting");
            std::process::exit(1);
        }
    }
}

pub fn create_config_file(loc: &PathBuf) -> (File, Config) {
    println!("Creating configuration file");
    let conf_file: File = match File::create(loc) {
        Err(e) => {
            println!("Unable to create configuration file: {:?}", e);
            std::process::exit(1);
        }
        Ok(file) => file
    };

    let master_dir: PathBuf = create_master_dir();
    (conf_file, create_trash_directories(master_dir))
}

fn create_trash_directories(master_dir: PathBuf) -> Config {
    let mut trash_loc = master_dir.clone();
    trash_loc.push("files");

    let mut trash_info = master_dir.clone();
    trash_info.push("info");

    Config::new(&master_dir)
}

fn create_master_dir() -> PathBuf {
    let mut master_dir: PathBuf;

    print!("Where would you like the trash directory to be? ");
    println!("Leave blank for the default locations:");
    println!("Lin: ~/.trash-rs/");
    println!("Win: C:\\Users\\your_name\\.trash-rs\\");
    println!("Mac: /Users/your_name/.trash-rs/");

    let mut desired_dir: String = String::new();
    match io::stdin().read_line(&mut desired_dir) {
        Err(e) => {
            println!("Error reading input: {:?}", e);
            println!("Exiting");
            std::process::exit(1);
        }
        _ => {
            if !(desired_dir.eq("\\n") || desired_dir.eq("\n") || desired_dir.is_empty()) {
                master_dir = PathBuf::from(desired_dir);
            } else {
                if let Some(d) = dirs_next::home_dir() {
                        master_dir = d;
                        master_dir.push(".trash-rs");
                } else {
                    println!("Could not resolve home directory. Please try again and specify a directory");
                    std::process::exit(1);
                }
            }
        }
    }

    match read_dir(&master_dir) {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("Could not find directory. Attempting to create it now");
                create_directories(&master_dir);
                println!("Trash directory created");
                master_dir
            }
            _ => {
                println!("Error while reading directory given: {:?}", e);
                std::process::exit(1);
            }
        } _ =>  master_dir
    }
}

fn create_directories(md: &PathBuf) {
    let mut tp = md.clone();
    tp.push("files");
    let mut ti = md.clone();
    ti.push("info");
   
    match fs::create_dir_all(tp) {
        Err(e) => {
            println!("Error while creating directory: {:?}", e);
            std::process::exit(1);
        } _ => {}
    }
    
    match fs::create_dir_all(ti) {
        Err(e) => {
            println!("Error while creating directory: {:?}", e);
            std::process::exit(1);
        } _ => {}
    }
}

fn parse_yn() -> bool {
    let mut c: String = String::new();
    match io::stdin().read_line(&mut c) {
        Err(e) => {
            println!("Error reading input: {:?}", e);
            std::process::exit(1);
        }
        _ => c.eq("y") || c.eq("Y")
    }
}
