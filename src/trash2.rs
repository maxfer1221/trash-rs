use serde::{Serialize, Deserialize};
use crate::config::Config;
use std::{io::prelude::*, fs::File};
use toml;

    
#[derive(Serialize, Deserialize)]
struct TrashFile {
    path: PathBuf,
    date: String,
}

#[derive(Serialize, Deserialize)]
struct TrashHandler {
    files: Vec<TrashCopies>,
}

#[derive(Serialize, Deserialize)]
struct TrashCopies {
    name: String,
    copies: i64,
}

fn read_file(file: PathBuf) -> &str {
    match File::open(file) {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("Trash files missing... do 'trash-rs clean -h' to create missing files. WARNING: Will delete all files currently in the trash");
                std::process::exit(1);
            } _ =>{ 
                println!("Error opening trash data file: {:?}", e);
            }
        } Ok(file) => {
            let mut buf_reader = BufReader::new(&file)
            let mut contents = String::new();
            
            match buf_reader.read_to_string(&mut contents) {
                Err(e) => {
                    println!("Error reading trash data file: {:?}", e);
                    std::process::exit(1);
                } _ => contents
            }
        }
    }
}

pub fn trash_contains(file_name: OsStr, master: PathBuf) -> (bool, u64) { 
    let mut s: String = match file_name.to_os_string().into_string() {
        Ok(string) => s = string,
        _ => {
            println!("Unable to read file name. Exiting");
            std::process::exit(1);
        },
    }

    let m: PathBuf = master.clone();
    md.push("metadata");
    md.set_extension("info");

    let metadata_as_str: &str = read_file(master);
    let th: TrashHandler = toml::from_str(metadata_as_str);

    for file in th.files {
        if file.name == file_name {
            (true, file.copies)
        }
    }
    (false, 0)
}

