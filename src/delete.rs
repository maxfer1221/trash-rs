use std::{path::PathBuf, env, time::SystemTime};
use std::fs::{self, rename};
use std::io::{Error, ErrorKind};
use crate::config::Config;
use crate::trash::{TrashFile};
use humantime;

macro_rules! err {
    ( $e:expr ) => {
        Error::new(ErrorKind::Other, format!("{:?}",$e));
    }
}

struct WriteOptions {
    pub overwrite_enabled: bool,
}

pub fn delete_files(files: Vec<String>, flags: Vec<String>, config: &Config) -> Result<(), Error> {
    let write_options: WriteOptions = WriteOptions { 
        overwrite_enabled: flags.iter().any(|f| f.contains("o")),
    };
    for file in files.iter().skip(2) {
        let (o, f): (PathBuf, PathBuf) =
            delete_file(file, &write_options, &config)?;
        write_metadata(&o, &f, &config)?;
    }
    Ok(())
}

fn delete_file(file: &String, wo: &WriteOptions, config: &Config) -> Result<(PathBuf, PathBuf), Error> { 
    let temp_buf: PathBuf; 
    temp_buf = PathBuf::from(&file);
    
    let mut fname: PathBuf = config.dirs.trash_dir.clone();
    fname.push(temp_buf.file_name().ok_or(err!("Could not find file name"))?);

    let stem: String = fname.file_stem().ok_or(err!("Could not find file stem"))?
                            .to_os_string().into_string().map_err(|e| err!(e))?;

    let ext: String = format!(".{}", match fname.extension() {
        Some(s) => s.to_os_string().into_string().map_err(|e| err!(e))?,
        None => String::new(),
    });
    
    let mut copy_count: u64 = 0;
    let overwrite_enabled: bool = wo.overwrite_enabled;

    while fname.exists() && !overwrite_enabled {
        copy_count += 1;
        
        fname.set_file_name(format!("{}({})", &stem, &copy_count));
        fname.set_extension(&ext);
    }

    let mut oname: PathBuf; 
    if temp_buf.is_absolute() {
        oname = temp_buf;
    } else { 
        oname = env::current_dir()?;
        oname.push(file);
    }

    rename(oname.clone(), fname.clone())?;
        
    Ok((oname, fname))
}

fn write_metadata(o: &PathBuf, f: &PathBuf, c: &Config) -> Result<(), Error> {
    let t: SystemTime = SystemTime::now();
    let mut time_as_string: String = humantime::format_rfc3339_seconds(t)
                                        .to_string().replace("T", " ");
    time_as_string.pop();

    let tf: TrashFile = TrashFile::new(&o, &time_as_string);
    let toml_as_string: String = toml::to_string(&tf).map_err(|e| err!(e))?;

    let mut final_file: PathBuf = c.dirs.trash_info.clone();
    final_file.push(f.file_name().ok_or(err!("Could not find file name"))?
                    .to_os_string().into_string().map_err(|e| err!(e))?);
    
    let final_ext: String = match final_file.extension() {
        Some(ext) => ext.to_os_string().into_string().map_err(|e| err!(e))?,
        None => String::new(),
    };

    final_file.set_extension(match final_ext.is_empty() {
        true => String::from("info"),
        false => format!("{}.{}", final_ext, "info"),
    });
    
    let bytes: &[u8] = toml_as_string.as_bytes();
    fs::write(final_file, bytes)?;

    Ok(())
}
