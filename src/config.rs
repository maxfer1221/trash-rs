use std::io::{self, BufReader, prelude::*, Error, ErrorKind};
use std::fs::{self, File};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub copy_ext: String,
    pub dirs: Directories,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Directories {    
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
            copy_ext: String::from("copy"),
            dirs: Directories {
                master_dir: master_dir.clone(),
                trash_dir: tp,
                trash_info: ti,
            },
        }
    }
}

pub fn fetch_config() -> Result<Config, Error> {
    let conf_loc: PathBuf = find_conf()?;

    match File::open(&conf_loc) {
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                let (file, config) = create_config_file(&conf_loc)?;
                write_config(&file, &config);
                Ok(config)
            } other => {
                println!("Error while opening configuration file: {:?}", other);
                println!("Create new configuration file? (y/n)");

                if parse_yn()? {
                    let (file, config) = create_config_file(&conf_loc)?;
                    write_config(&file, &config);
                    Ok(config)
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

            Ok(toml::from_str(&contents)?)
        }
    }
}

fn write_config(mut file: &File, config: &Config) -> Result<(), Error> {
    let toml_as_str: String = toml::to_string(config)
        .unwrap_or(Err(Error::new(ErrorKind::Other, "Could not serialize config"))?);
    let bytes: &[u8] = (toml_as_str).as_bytes();
    file.write(bytes)?;
    Ok(())
}

pub fn find_conf() -> Result<PathBuf, Error> {
    let mut buf: PathBuf = match dirs_next::config_dir() {
        Some(d) => d,
        _ => return Err(std::io::Error::new(
                ErrorKind::NotFound, "Could not find configuration directory")),
    };
    buf.push("trash-rs");
    buf.push("config");
    buf.set_extension("toml");
    Ok(buf)
}

pub fn create_config_file(loc: &PathBuf) -> Result<(File, Config), Error> {
    println!("Creating configuration file");
    let conf_file: File = match File::create(loc) {
        Err(e) => {
            println!("Unable to create configuration file: {:?}", e);
            std::process::exit(1);
        }
        Ok(file) => file
    };

    println!("Create trash directories again? WARNING: will overwrite old directories (y/n)");
    let master_dir: PathBuf = create_master_dir()?;
    let config = Config::new(&master_dir);


    write_config(&conf_file, &config);

    Ok((conf_file, config))
}

// pub fn recreate_master_dir() {}

pub fn create_master_dir() -> Result<PathBuf, Error> {
    let mut master_dir: PathBuf;

    println!("Where would you like the trash directory to be? ");
    println!("Leave blank for the default locations:");
    println!("Lin: ~/.trash-rs/");
    println!("Win: C:\\Users\\your_name\\.trash-rs\\");
    println!("Mac: /Users/your_name/.trash-rs/");

    let mut desired_dir: String = String::new();
    io::stdin().read_line(&mut desired_dir)?;
    if !(desired_dir.eq("\\n") || desired_dir.eq("\n") || desired_dir.is_empty()) {
        master_dir = PathBuf::from(desired_dir);
    } else {
        master_dir = match dirs_next::home_dir() {
            Some(p) => p,
            None => {
                println!("Could not resolve home directory");
                std::process::exit(1);
            }
        };
        master_dir.push(".trash-rs");
    }

    create_directories(&master_dir)?;
    println!("Trash directory created");
    Ok(master_dir)
}

fn create_directories(md: &PathBuf) -> Result<(), Error> {
    println!("{:?}", md);

    let mut tp = md.clone();
    tp.push("files");
    let mut ti = md.clone();
    ti.push("info");
    let mut meta = md.clone();
    meta.push("metadata");
    meta.set_extension("info");

    fs::create_dir_all(tp)?; 
    fs::create_dir_all(ti)?;
    File::create(meta)?;
    Ok(())
}

fn parse_yn() -> Result<bool, Error> {
    let mut c: String = String::new();
    io::stdin().read_line(&mut c)?;
    Ok(c.eq("y") || c.eq("Y"))
}
