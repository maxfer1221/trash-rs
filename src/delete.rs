use std::{ffi::OsStr, path::PathBuf, env, time::SystemTime};
use std::fs::{self, rename, File};
use std::io::{Error, Read, BufReader};
use crate::config::Config;
use crate::trash::{TrashFile, TrashCopies, TrashHandler, trash_contains};
use humantime;

trait BufUtil {
    fn is_copy_obj(&self, config: &Config) -> bool;
    fn rename_dup(self, copy_ext: &String, file_count: u64) -> PathBuf;
    fn get_full_stem(&self, c: &Config) -> Result<PathBuf, Error>;
}

impl BufUtil for PathBuf {
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

    fn get_full_stem(&self, c: &Config) -> Result<PathBuf, Error> {
        let mut parent: PathBuf = match self.parent() {
            Some(p) => p.to_path_buf(),
            _ => PathBuf::new(),
        };

        if self.is_copy_obj(c) {
            let stem: PathBuf = match self.file_stem() {
                Some(s) => PathBuf::from(s),
                _ => PathBuf::new(),
            };
            parent.push(stem);
            return Ok(parent.to_path_buf());
        }
        let stem: PathBuf = match self.file_name() {
            Some(d) => PathBuf::from(d),
            _ => PathBuf::new(),
        };
        parent.push(stem);
        Ok(parent.to_path_buf())
    }
}

pub fn delete_files(files: Vec<String>, config: &Config) -> Result<(), Error> {
    for file in files.iter().skip(2) {
        let (o, f): (PathBuf, PathBuf) = delete_file(file, &config)?;
        write_metadata(&o, &f, &config)?;
        write_master_metadata(&f, &config)?;
    }
    Ok(())
}

fn delete_file(file: &String, config: &Config) -> Result<(PathBuf, PathBuf), Error> { 
    let mut oname: PathBuf; 
    let mut fname: PathBuf;
    let temp_buf: PathBuf;
    
    temp_buf = PathBuf::from(&file);
    fname = config.dirs.trash_dir.clone();
    fname.push(temp_buf.file_name().unwrap());
   
    println!("{:?}", fname.is_copy_obj(&config));

    if fname.is_copy_obj(config) {
        let n = fname.clone();
        let index: u64 = trash_contains(match n.file_name() {
            Some(n) => n,
            _ => OsStr::new(""),
        }, &config.dirs.master_dir).1;
        fname = fname.rename_dup(&config.copy_ext, index);
    } else {
        let (tc, c): (bool, u64) = trash_contains(match fname.file_name() {
            Some(n) => n,
            _ => OsStr::new(""),
        }, &config.dirs.master_dir);

        fname = match tc {
            true => fname.rename_dup(&config.copy_ext, c),
            false => fname,
        };
    }

    oname = env::current_dir()?;
    oname.push(file);

    match rename(oname.clone(), fname.clone()) {
        Ok(_) => return Ok((oname, fname)),
        Err(e) => {
            println!("fname={:?}, oname={:?}", oname, fname);
            return Err(e);
        }
    }
}

fn write_metadata(o: &PathBuf, f: &PathBuf, c: &Config) -> Result<(), Error> {
    let t: SystemTime = SystemTime::now();
    let mut time_as_string: String = humantime::format_rfc3339_seconds(t)
                                        .to_string().replace("T", " ");
    time_as_string.pop();

    let tf: TrashFile = TrashFile::new(&o, &time_as_string);
    let toml_as_string: String = match toml::to_string(&tf) {
        Ok(s) => s,
        Err(e) => {
            println!("Could not serialize metadata: {:?}", e);
            std::process::exit(1);
        }
    };

    let mut final_file: PathBuf = c.dirs.trash_info.clone();
    final_file.push(match f.file_name() {
        Some(n) => match n.to_os_string().into_string() {
            Ok(s) => s,
            Err(os) => {
                println!("Could not convert OsString: {:?}", os);
                std::process::exit(1);
            }
        },
        _ => String::new(),
    });
    
    let final_ext: String = match final_file.extension() {
        Some(ext) => match ext.to_os_string().into_string() {
            Ok(s) => s,
            Err(e) => {
                println!("Could not resolve file extension: {:?}", e);
                std::process::exit(1);
            },
        },
        None => String::new(),
    };

    if !final_ext.is_empty() {
        final_file.set_extension(format!("{}.{}", final_ext, "info"));
    } else {
        final_file.set_extension("info");
    }
    
    let bytes: &[u8] = toml_as_string.as_bytes();
    fs::write(final_file, bytes)?;

    Ok(())
}

fn write_master_metadata(f: &PathBuf, c: &Config) -> Result<(), Error> {
    let mut fdir: PathBuf = c.dirs.master_dir.clone();
    fdir.push("metadata");
    fdir.set_extension("info");

    let file: File = File::open(&fdir)?;
    let mut buf_reader = BufReader::new(&file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents)?;

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
    println!("th: {:?}", th);
    let stem: String = match f.file_stem() {
        Some(s) => match s.to_os_string().into_string() {
            Ok(s) => s,
            Err(e) => return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Unsupported, "Could not get file stem"))
        },
        None => String::new(),
    };

    let mut present: bool = false;
    for mut file in &mut th.files {
        println!("{:?}", file);
        let fstr: String = stem.clone();
        // .into_os_string().into_string() {
        //     Ok(s) => s,
        //     Err(s) => {
        //         println!("Could not convert file to string: {:?}", s);
        //         std::process::exit(1);
        //     },
        // };
        println!("fstr: {:?}", &fstr);
        println!("file.name: {:?}", &file.name);
        println!("file: {:?}", &file);
        println!("{}", fstr.eq(&file.name));
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
    fs::write(fdir, bytes)?;
    Ok(())
}
