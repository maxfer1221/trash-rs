use std::{ffi::OsStr, path::{Path, PathBuf}, env, time::SystemTime};
use std::fs::{self, rename, File};
use std::io::{Read, BufReader};
use crate::config::Config;
use crate::trash::{TrashFile, TrashCopies, TrashHandler, trash_contains};
use humantime;

trait Rename {
    fn is_copy_obj(&self, config: &Config) -> bool;
    fn rename_dup(self, copy_ext: &String, file_count: u64) -> PathBuf;
}

impl Rename for PathBuf {
    fn rename_dup(mut self, copy_ext: &String, file_count: u64) -> PathBuf {
        self.set_extension(
            format!("{}{}", copy_ext, file_count)
            );
        self
    }

    fn is_copy_obj(&self, config: &Config) -> bool {
        let ext: String = match self.extension() {
            Some(s) => match s.to_os_string().into_string() {
                Ok(s) => s,
                _ => "".into(),
            },
            _ => "".into(),
        };

        if ext.eq("") {
            return false
        }
        
        println!("{} {}", config.copy_ext, &ext);
        if &ext[..(config.copy_ext.len())] == config.copy_ext {
            return true
        } false
    }
}

pub fn delete_files(files: Vec<String>, config: &Config) -> std::io::Result<()> {
    for file in files.iter().skip(2) {
        match delete_file(file, &config) {
            Err(e) => {
                println!("Error moving file: {:?}", e);
            },
            Ok((o, f)) => write_metadata(&o, &f, &config),
        }
    }
    Ok(())
}

fn delete_file(file: &String, config: &Config) -> std::io::Result<(PathBuf, PathBuf)> { 
    let mut oname: PathBuf; 
    let mut fname: PathBuf;
    let temp_buf: PathBuf;
    
    temp_buf = PathBuf::from(&file);
    fname = config.dirs.trash_dir.clone();
    fname.push(temp_buf.file_name().unwrap());
    
    if fname.is_copy_obj(config) {
        fname = match fname.clone().file_name() {
            Some(n) => {
                fname.rename_dup(
                    &config.copy_ext, 
                    trash_contains(n, &config.dirs.master_dir).1)
            },
            None => PathBuf::new().rename_dup(
                &config.copy_ext, trash_contains(
                    OsStr::new(""), &config.dirs.trash_info).1),
        };
    } else {
        let (tc, c): (bool, u64) = trash_contains(
            match fname.file_name(){
                Some(n) => n,
                None => OsStr::new(""),
            }, &config.dirs.master_dir);

        fname = match tc {
            true => fname.rename_dup(&config.copy_ext, c),
            false => fname,
        };
    }

    oname = env::current_dir().unwrap();
    oname.push(file);

    match rename(oname.clone(), fname.clone()) {
        Ok(_) => return Ok((oname, fname)),
        Err(e) => Err(e),
    }
}

fn write_metadata(o: &PathBuf, f: &PathBuf, c: &Config) /*-> std::io::Result<std::io::Error>*/ {
    let t: SystemTime = SystemTime::now();
    let mut time_as_string: String = humantime::format_rfc3339_seconds(t)
                                        .to_string().replace("T", " ");
    time_as_string.pop();

    let tf: TrashFile = TrashFile::new(&o, &time_as_string);
    match toml::to_string(&tf) {
        Ok(s) => {
            let mut final_file: PathBuf = f.clone();
            final_file.set_extension(
                format!("{}.{}", 
                match final_file.extension() {
                    Some(ext) => match ext.to_os_string().into_string() {
                        Ok(s) => s,
                        Err(e) => {
                            println!("Could not resolve file extension: {:?}", e);
                            std::process::exit(1);
                        },
                    },
                    None => {
                        println!("Could not resolve file extension");
                        std::process::exit(1);
                    }
                }, "info"));
            let bytes: &[u8] = s.as_bytes();
            match fs::write(final_file, bytes) {
                Err(e) => {
                    println!("Error writing to metadata file: {:?}", e);
                    std::process::exit(1);
                }, _ => {}
            }
        } 
        Err(_e) => {},
    }

    let mut fdir: PathBuf = c.dirs.master_dir.clone();
    fdir.push("metadata");
    fdir.set_extension("info");

    let file: File = match File::open(&fdir){
        Ok(f) => f,
        Err(e) => {
            println!("Could not open metadata file: {:?}", e);
            std::process::exit(1);
        }
    };
    let mut buf_reader = BufReader::new(&file);
    let mut contents = String::new();

    match buf_reader.read_to_string(&mut contents) {
        Err(error) => {
            println!("Error reading metadata file: {:?}", error);
            println!("Exiting");
            std::process::exit(1);
        },
        _ => {},
    }
    let mut th: TrashHandler;
    let tc: TrashCopies = TrashCopies::new(match f.file_name() {
        Some(n) => match n.to_os_string().into_string() {
            Ok(s) => s,
            Err(e) => {
                println!("Could not resolve metadata file: {:?}", e);
                std::process::exit(1);
            }
        },
        None => String::new(),
    });
    if contents.is_empty() {
        th = TrashHandler::new();
    } else {
        th = match toml::from_str(&contents) {
            Ok(t) => t,
            Err(e) => {
                println!("Unable to deserialize metadata file: {:?}", e);
                std::process::exit(1);
            }
        };
    }
    
    
    let stem: PathBuf = match f.is_copy_obj(c) {
        true => match f.file_stem() {
            Some(s) => {
                let r: &Path = match f.parent() {
                    Some(d) => d,
                    None => {
                        println!("Could not resolve file parent");
                        std::process::exit(1);
                    },
                };
                let mut x: PathBuf = r.to_path_buf();
                x.push(s);
                x
            },
            None => {
                let r: &Path = match f.parent() {
                    Some(d) => d,
                    None => {
                        println!("Could not resolve parent path to metadata file");
                        std::process::exit(1);
                    },
                };
                let x: PathBuf = r.to_path_buf();
                x
            }
        },
        false => match f.file_name() {
            Some(s) => { 
                let r: &Path = match f.parent() {
                    Some(d) => d,
                    None => {
                        println!("Could not resolve file parent");
                        std::process::exit(1);
                    },
                };
                let mut x: PathBuf = r.to_path_buf();
                x.push(s);
                x
            },
            None => {
                let r: &Path = match f.parent() {
                    Some(d) => d,
                    None => {
                        println!("Could not resolve file name");
                        std::process::exit(1);
                    },
                };
                let x: PathBuf = r.to_path_buf();
                x
            }
        },
    };

    let mut present: bool = false;
    for mut file in &mut th.files {
        let fstr: String = match stem.clone().into_os_string().into_string() {
            Ok(s) => s.clone(),
            Err(e) => {
                println!("Could not resolve stem: {:?}", e);
                std::process::exit(1);
            }
        };
        if fstr.eq(&file.name) {
            file.copies += 1;
            present = true;
            break;
        }
    } if !present {
        th.push(tc);
    }

    let pre_bytes: String = match toml::to_string(&th) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to serialze metadata: {:?}", e);
            std::process::exit(1);
        },
    };
    let bytes = pre_bytes.as_bytes();
    match fs::write(fdir, bytes) {
        Err(e) => {
            println!("Couldn't write to metadata file: {:?}", e);
            std::process::exit(1);
        } _ => {}
    }
    // Ok(())
}
