use serde::{Serialize, Deserialize};
use std::{ffi::OsStr, fs::File, path::PathBuf};
use std::io::{BufReader, ErrorKind, prelude::*};
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
    copies: u64,
}


fn read_file(file: PathBuf) -> String {
    println!("{:?}", file);
    match File::open(file) {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("Trash files missing... do 'trash-rs clean -h' to create missing files. WARNING: Will delete all files currently in the trash");
                std::process::exit(1);
            } _ =>{ 
                println!("Error opening trash data file: {:?}", e);
                std::process::exit(1);
            }
        } Ok(file) => {
            let mut buf_reader = BufReader::new(&file);
            let mut contents: String = String::new();
            
            match buf_reader.read_to_string(&mut contents) {
                Err(e) => {
                    println!("Error reading trash data file: {:?}", e);
                    std::process::exit(1);
                } _ => contents
            }
        }
    }
}

pub fn trash_contains(file_name: &OsStr, master: &PathBuf) -> (bool, u64) { 
    let s: String = match file_name.to_os_string().into_string() {
        Ok(string) => string, 
        _ => {
            println!("Unable to read file name. Exiting");
            std::process::exit(1);
        },
    };

    let mut md: PathBuf = master.clone();
    md.push("metadata");
    md.set_extension("info");

    let metadata: String = read_file(md);
    if metadata.is_empty() {
        return (false, 0);
    }
    let th: TrashHandler = match toml::from_str(metadata.as_str()) {
        Err(e) => {
            println!("{:?}", e);
            println!("Unable to deserialize trash metadata. Exiting");
            std::process::exit(1);
        } Ok(t) => t,
    };

    for file in th.files {
        if file.name.eq(&s) {
                return (true, file.copies)
        }
    }
    (false, 0)
}
