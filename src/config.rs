use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
//use toml;
//use dirs_next;

#[derive(Serialize, Deserialize)]
pub struct Config {
    trash_dir: String,
}

impl Config {
    fn new() -> Config {
        return Config { trash_dir: String::new() };
    }
}

pub fn create_config() -> Config {
    let (conf_loc, trash_loc) = resolve_dirs();
    let mut config: Config = Config::new();

    let mut conf_file = File::open(&conf_loc);
    
    match conf_file {
        Err(file_open_error) => {
            println!("File not found. Creating configuration file.");     
            conf_file = File::create(&conf_loc);
            let mut conf_file = match conf_file {
                Err(file_create_error) => {
                    panic!("Unable to create configuration file: {:?}", file_create_error);
                }
                Ok(file) => file
            };

            config = Config { trash_dir: trash_loc.into_os_string().into_string().unwrap() };
            let toml_as_str: String = toml::to_string(&config).unwrap();
            let bytes: &[u8] = (toml_as_str).as_bytes();
            conf_file.write(bytes);
            return config;
        }
        Ok(file) => {
            println!("{:?}", file);

            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();

            buf_reader.read_to_string(&mut contents);
            println!("File contents: {}", contents);

            config = Config { trash_dir: String::new() };
            return config;
        }
    }
}

fn resolve_dirs() -> (PathBuf, PathBuf)  {
    let     conf_temp: Option<PathBuf> = dirs_next::config_dir();
    let mut conf_true: PathBuf;
    match conf_temp {
        Some(buf) => {
            conf_true = buf;
            conf_true.push("trash-rs");
            conf_true.push("config");
            conf_true.set_extension("toml");
        }
        None => {
            println!("OS \"config\" location unkown. Please specify a directory to hold configuration files.");
            let mut str_to_buf = String::new();
            io::stdin().read_line(&mut str_to_buf);
            conf_true = PathBuf::from(str_to_buf);
        }
    }
    
    let     trash_temp: Option<PathBuf> = dirs_next::home_dir();
    let mut trash_true: PathBuf;
    match trash_temp {
        Some(buf) => {
            trash_true = buf;
            trash_true.push(".trash-rs");
        }
        None => {
            println!("OS \"home\" locatpion unkown. Please specify a destination for removed files.");
            let mut str_to_buf = String::new();
            let read_result = io::stdin().read_line(&mut str_to_buf);
            match read_result {
                Err(io_err) => panic!("Failed to read input: {:?}", io_err),
                _ => {}
            }
            trash_true = PathBuf::from(str_to_buf);
        }
    }
    
    (conf_true, trash_true)
}
