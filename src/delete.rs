use std::{path::PathBuf, env, time::SystemTime};
use std::fs::{self, rename};
// use std::io::{Read, BufReader};
use std::io::{Error, ErrorKind};
use crate::config::Config;
use crate::trash::{TrashFile};
use humantime;

macro_rules! err {
    ( ) => {
        Error::new(ErrorKind::Other, "");
    }
}

pub fn delete_files(files: Vec<String>, config: &Config) -> Result<(), Error> {
    for file in files.iter().skip(2) {
        let (o, f): (PathBuf, PathBuf) = delete_file(file, &config)?;
        write_metadata(&o, &f, &config)?;
        // write_master_metadata(&f, &config)?;
    }
    Ok(())
}

fn delete_file(file: &String, config: &Config) -> Result<(PathBuf, PathBuf), Error> { 
    let temp_buf: PathBuf; 
    temp_buf = PathBuf::from(&file);
    
    let mut fname: PathBuf = config.dirs.trash_dir.clone();
    fname.push(temp_buf.file_name().ok_or(err!())?);

    let mut copy_count: u64 = 0;
    
    let stem: String = fname.file_stem().ok_or(err!())?.to_os_string()
                            .into_string().map_err(|_e| err!())?;

    let ext: String = format!(".{}", match fname.extension() {
        Some(s) => s.to_os_string().into_string().map_err(|_e| err!())?,
        None => String::new(),
    });
    
    let no_overwrite: bool = true;
    while fname.exists() || no_overwrite {
        copy_count += 1;
        fname.set_file_name(format!("{}({})", stem, copy_count));
    } 

    let mut oname: PathBuf; 
    oname = env::current_dir()?;
    oname.push(file);

    rename(oname.clone(), fname.clone())?;
        
    Ok((oname, fname))
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
