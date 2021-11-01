use std::io::{self, BufReader, prelude::*};
use std::fs::File;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
//use toml;
//use dirs_next;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub trash_dir: PathBuf,
}

impl Config {
    fn new() -> Config {
        return Config { trash_dir: PathBuf::new() };
    }
}

pub fn create_config() -> Config {
    let (conf_loc, trash_loc) = resolve_dirs();
    let config: Config;

    let mut conf_file = File::open(&conf_loc);

    match conf_file {
        Err(_file_open_error) => {
            println!("File not found. Creating configuration file.");     
            conf_file = File::create(&conf_loc);
            let mut conf_file = match conf_file {
                Err(file_create_error) => {
                    panic!("Unable to create configuration file: {:?}", file_create_error);
                }
                Ok(file) => file
            };

            config = Config { trash_dir: trash_loc };
            let toml_as_str: String = toml::to_string(&config).unwrap();
            let bytes: &[u8] = (toml_as_str).as_bytes();
            let write_success = conf_file.write(bytes);
            match write_success {
                Err(write_error) => {
                    println!("Error writing to config file: {:?}", write_error);
                }
                _ => {}
            }
            return config;
        }
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();

            let read_success = buf_reader.read_to_string(&mut contents);
            match read_success {
                Err(read_error) => {
                    println!("Error reading configuration file: {:?}", read_error);
                    return Config::new();
                },
                _ => {
                    let conversion = toml::from_str(&contents);
                    match conversion {
                        Err(deserialize_error) => {
                            println!("Deserialization error: {:?}", deserialize_error);
                            return Config::new();
                        },
                        Ok(conf) => return conf,
                    }
                }
            }
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
            let io_success = io::stdin().read_line(&mut str_to_buf);
            match io_success {
                Err(io_err) => {
                    println!("Error reading input: {:?}", io_err);
                }
                _ => {}
            }
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
