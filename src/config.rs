//use std::{fs, path::PathBuf};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
//use toml;
//use dirs_next;


#[derive(Serialize, Deserialize)]
struct Config {
    dir: String,
}

pub fn create_config() {
    let config_loc: Option<PathBuf> = dirs_next::config_dir();
    let mut app_conf:   PathBuf = PathBuf::new();
    match config_loc {
        Some(buf) => {
            app_conf = buf;
            println!("{:?}", app_conf);
        }
        None => {
            println!("Config location unkown. Exiting.");
        }
    };
    app_conf.push("trash-rs");
    app_conf.push("config");
    app_conf.set_extension("toml");
    /*
    if let fs::read_to_string(config_loc) = Ok(String { vec: val }) {
        match toml::from_str(fs::read_to_string(config_loc)) { 
            Ok(config) => println!("{:?}", config);
            Err(e) => return Config { dir: "" };
        }
    };
    */
}

